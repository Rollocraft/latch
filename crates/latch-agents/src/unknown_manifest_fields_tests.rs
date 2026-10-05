use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn unknown_fields_are_rejected_at_every_layer() {
    for pointer in [
        "",
        "/agent",
        "/identity",
        "/requires/0",
        "/requires/0/resource",
    ] {
        let mut value = input();
        value
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("allow".into(), json!(true));
        assert!(
            serde_json::from_value::<AgentManifest>(value).is_err(),
            "{pointer}"
        );
    }
    let mut value = input();
    value["agent"]["model_metadata"] =
        json!({"provider":"local", "model":"coder", "version":"1", "trust_level":100});
    assert!(serde_json::from_value::<AgentManifest>(value).is_err());
    let mut value = input();
    value["identity"]["provider"] = json!("local");
    assert!(serde_json::from_value::<AgentManifest>(value).is_err());
}
