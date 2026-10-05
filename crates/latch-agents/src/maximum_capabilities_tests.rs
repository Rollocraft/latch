use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn maximum_capability_count_round_trips() {
    let requests = (0..MAX_CAPABILITIES)
        .map(|index| capability("file.read", &format!("file:///project/{index}"), false))
        .collect();
    let manifest = manifest(requests);
    assert_eq!(manifest.requires().len(), MAX_CAPABILITIES);
    assert_eq!(
        AgentManifest::from_json(&manifest.to_json().unwrap()).unwrap(),
        manifest
    );
}
