use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn initial_removal_and_version_only_updates() {
    assert!(!PermissionDiff::initial(&manifest(vec![])).requires_review());
    assert!(PermissionDiff::initial(&sample()).requires_review());
    let removal = PermissionDiff::between(&sample(), &manifest(vec![])).unwrap();
    assert!(!removal.requires_review());
    assert!(removal.added().is_empty());
    assert_eq!(removal.removed().len(), 1);
    let mut value = input();
    value["agent"]["version"] = json!("2.0.0");
    let next: AgentManifest = serde_json::from_value(value).unwrap();
    assert!(
        PermissionDiff::between(&sample(), &next)
            .unwrap()
            .is_empty()
    );
    assert_eq!(next.agent().version(), "2.0.0");
    assert_eq!(next.schema_version(), 1);
}
