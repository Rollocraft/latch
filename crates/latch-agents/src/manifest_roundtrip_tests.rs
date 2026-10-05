use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn validated_json_round_trips_and_empty_permissions_are_explicit() {
    for manifest in [sample(), manifest(vec![])] {
        assert_eq!(
            AgentManifest::from_json(&manifest.to_json().unwrap()).unwrap(),
            manifest
        );
        assert_eq!(
            serde_json::from_value::<AgentManifest>(serde_json::to_value(&manifest).unwrap())
                .unwrap(),
            manifest
        );
    }
    let mut value = input();
    value.as_object_mut().unwrap().remove("requires");
    assert!(serde_json::from_value::<AgentManifest>(value).is_err());
    assert!(!manifest(vec![]).requests(
        &ActionName::new("file.read").unwrap(),
        &ResourceId::new("file:///project/src").unwrap(),
        "development"
    ));
}
