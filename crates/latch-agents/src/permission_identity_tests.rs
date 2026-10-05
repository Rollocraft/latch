use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn different_agent_or_binding_cannot_reuse_a_permission_baseline() {
    for pointer in [
        "/agent/name",
        "/identity/owner",
        "/identity/team",
        "/identity/device",
        "/identity/runtime",
        "/identity/purpose",
    ] {
        let mut value = input();
        *value.pointer_mut(pointer).unwrap() = json!("other");
        let next: AgentManifest = serde_json::from_value(value).unwrap();
        assert_eq!(
            PermissionDiff::between(&sample(), &next),
            Err(ManifestError::DifferentAgent)
        );
    }
    let mut value = input();
    value["identity"]["environment"] = json!("production");
    value["requires"][0]["environment"] = json!("production");
    let next: AgentManifest = serde_json::from_value(value).unwrap();
    assert_eq!(
        PermissionDiff::between(&sample(), &next),
        Err(ManifestError::DifferentAgent)
    );
}
