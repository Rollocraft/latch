use crate::workflow_test_fixtures::*;
use crate::{bounded_file_read::bounded_read, policy_document_read::load_policy};
use latch_policy::MAX_POLICY_DOCUMENT_BYTES;
use serde_json::json;

#[test]
fn policy_validation_supports_json_and_both_yaml_extensions() {
    let files = Files::new();
    for (name, text) in [
        (
            "policy.json",
            policy_value(json!({"kind":"allow"})).to_string(),
        ),
        ("policy.yaml", "version: 1\npolicies: []\n".into()),
        ("policy.yml", "version: 1\npolicies: []\n".into()),
    ] {
        let path = files.write(name, text);
        let (result, output) = call(&["policy".as_ref(), "validate".as_ref(), &path]);
        assert!(result.is_ok(), "{result:?}");
        assert!(output.contains("valid policy document v1"));
    }
}

#[test]
fn invalid_policy_documents_and_bounded_reads_fail_without_success_output() {
    let files = Files::new();
    for (name, bytes) in [
        ("bad.json", b"{".to_vec()),
        ("version.json", br#"{"version":2,"policies":[]}"#.to_vec()),
        (
            "unknown.json",
            br#"{"version":1,"policies":[],"allow":true}"#.to_vec(),
        ),
        (
            "duplicate.json",
            br#"{"version":1,"version":1,"policies":[]}"#.to_vec(),
        ),
        ("bad.yaml", b"version: 1\npolicies: [".to_vec()),
        (
            "multi.yaml",
            b"version: 1\npolicies: []\n---\nversion: 1\npolicies: []".to_vec(),
        ),
        ("oversize.json", vec![b' '; MAX_POLICY_DOCUMENT_BYTES + 1]),
        ("extension.txt", br#"{"version":1,"policies":[]}"#.to_vec()),
    ] {
        let path = files.write(name, bytes);
        let (result, output) = call(&["policy".as_ref(), "validate".as_ref(), &path]);
        assert!(result.is_err(), "{name}");
        assert!(output.is_empty());
    }
    let missing = files.0.join("missing.json").into_os_string();
    assert!(load_policy(&missing).is_err());
    assert!(bounded_read(&files.0.clone().into_os_string(), 16).is_err());
    let path = files.write("bounded", b"1234");
    assert_eq!(bounded_read(&path, 4).unwrap(), b"1234");
    assert!(bounded_read(&path, 3).unwrap_err().contains("byte limit"));
}
