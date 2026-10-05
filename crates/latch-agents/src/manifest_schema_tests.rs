use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn schema_and_required_fields_are_strict() {
    for version in [
        json!(0),
        json!(2),
        json!(-1),
        json!(1.0),
        json!("1"),
        json!(null),
    ] {
        let mut value = input();
        value["schema_version"] = version;
        assert!(serde_json::from_value::<AgentManifest>(value).is_err());
    }
    for pointer in [
        "",
        "/agent",
        "/identity",
        "/requires/0",
        "/requires/0/resource",
    ] {
        let original = input();
        let fields: Vec<_> = original
            .pointer(pointer)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        for field in fields {
            if field == "model_metadata" {
                continue;
            }
            let mut value = original.clone();
            value
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(&field);
            assert!(
                serde_json::from_value::<AgentManifest>(value).is_err(),
                "{pointer}/{field}"
            );
        }
    }
    assert_eq!(
        AgentManifest::new(
            2,
            sample().agent().clone(),
            sample().identity().clone(),
            vec![]
        ),
        Err(ManifestError::UnsupportedSchemaVersion(2))
    );
}
