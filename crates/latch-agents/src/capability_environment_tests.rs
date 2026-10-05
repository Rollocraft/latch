use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn capability_environment_is_exact_and_must_match_binding() {
    for environment in ["", "*", "dev/*", " development"] {
        assert!(
            RequestedCapability::new(
                ActionName::new("file.read").unwrap(),
                sample().requires().first().unwrap().resource().clone(),
                environment
            )
            .is_err()
        );
    }
    let mut value = input();
    value["requires"][0]["environment"] = json!("production");
    assert!(serde_json::from_value::<AgentManifest>(value).is_err());
    let request = sample().requires().first().unwrap().clone();
    assert_eq!(request.action().as_str(), "file.read");
    assert!(!request.requests(
        request.action(),
        request.resource().resource(),
        "production"
    ));
}
