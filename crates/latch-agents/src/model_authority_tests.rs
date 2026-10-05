use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn model_metadata_is_not_runtime_identity_or_authority() {
    let mut value = input();
    value["agent"]["model_metadata"] =
        json!({"provider":"arbitrary-vendor", "model":"claimed-model", "version":"999"});
    let manifest: AgentManifest = serde_json::from_value(value).unwrap();
    let bound = manifest.bind(&identity(), 20).unwrap();
    assert_eq!(bound.identity(), &identity());
    assert_eq!(bound.manifest(), &manifest);
    assert!(bound.requests_action(&session(), &action(), 20).unwrap());
    assert!(
        PermissionDiff::between(&sample(), &manifest)
            .unwrap()
            .is_empty()
    );
    let metadata = manifest.agent().model_metadata().unwrap();
    assert_eq!(metadata.provider(), "arbitrary-vendor");
    assert_eq!(metadata.model(), "claimed-model");
    assert_eq!(metadata.version(), "999");
    let mut runtime = identity();
    runtime.provider = "another-vendor".into();
    runtime.model = "other-model".into();
    runtime.model_version = "2".into();
    runtime.trust_level = 100;
    assert!(manifest.bind(&runtime, 20).is_ok());
}
