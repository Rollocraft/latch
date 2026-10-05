use crate::workflow_test_fixtures::*;
use latch_agents::MAX_MANIFEST_BYTES;
use serde_json::json;

#[test]
fn manifests_reject_invalid_oversized_and_different_identity_inputs() {
    let files = Files::new();
    let old = files.write("old.json", manifest_value().to_string());
    for bytes in [
        b"{}".to_vec(),
        b"schema_version: 1".to_vec(),
        vec![0xff],
        vec![b' '; MAX_MANIFEST_BYTES + 1],
    ] {
        let invalid = files.write("invalid.json", bytes);
        for args in [
            vec!["manifest".as_ref(), "check".as_ref(), invalid.as_os_str()],
            vec![
                "manifest".as_ref(),
                "diff".as_ref(),
                old.as_os_str(),
                invalid.as_os_str(),
            ],
            vec![
                "manifest".as_ref(),
                "diff".as_ref(),
                invalid.as_os_str(),
                old.as_os_str(),
            ],
        ] {
            let (result, output) = call(&args);
            assert!(result.is_err());
            assert!(output.is_empty());
        }
    }
    for (pointer, replacement) in [
        ("/schema_version", json!(2)),
        ("/requires/0/action", json!("invalid")),
        ("/requires/0/environment", json!("production")),
    ] {
        let mut value = manifest_value();
        *value.pointer_mut(pointer).unwrap() = replacement;
        let invalid = files.write("invalid.json", value.to_string());
        assert!(
            call(&["manifest".as_ref(), "check".as_ref(), &invalid])
                .0
                .is_err()
        );
    }
    let mut different = manifest_value();
    different["identity"]["owner"] = json!("someone-else");
    let different = files.write("different.json", different.to_string());
    assert!(
        call(&["manifest".as_ref(), "diff".as_ref(), &old, &different])
            .0
            .unwrap_err()
            .contains("DifferentAgent")
    );
}
