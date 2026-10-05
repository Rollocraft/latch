use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn duplicate_json_fields_and_trailing_data_are_rejected() {
    let text = sample().to_json().unwrap();
    for (old, new) in [
        (
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
        ),
        (
            "\"name\":\"backend-developer\"",
            "\"name\":\"backend-developer\",\"name\":\"other\"",
        ),
        (
            "\"action\":\"file.read\"",
            "\"action\":\"file.read\",\"action\":\"file.write\"",
        ),
        (
            "\"kind\":\"subtree\"",
            "\"kind\":\"subtree\",\"kind\":\"exact\"",
        ),
    ] {
        assert!(text.contains(old));
        assert!(AgentManifest::from_json(&text.replace(old, new)).is_err());
    }
    assert!(AgentManifest::from_json(&(text + " {}")).is_err());
}
