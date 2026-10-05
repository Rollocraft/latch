use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn duplicate_capabilities_and_size_limits_are_rejected() {
    let request = sample().requires().first().unwrap().clone();
    assert_eq!(
        AgentManifest::new(
            1,
            sample().agent().clone(),
            sample().identity().clone(),
            vec![request.clone(), request.clone()]
        ),
        Err(ManifestError::DuplicateCapability)
    );
    let mut value = input();
    value["requires"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::to_value(&request).unwrap());
    assert!(serde_json::from_value::<AgentManifest>(value).is_err());
    assert_eq!(
        AgentManifest::new(
            1,
            sample().agent().clone(),
            sample().identity().clone(),
            vec![request; crate::manifest::MAX_CAPABILITIES + 1]
        ),
        Err(ManifestError::TooManyCapabilities)
    );
    assert!(
        AgentManifest::from_json(&" ".repeat(crate::manifest::MAX_MANIFEST_BYTES + 1)).is_err()
    );
}
