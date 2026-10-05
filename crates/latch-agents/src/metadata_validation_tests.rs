use crate::manifest_test_fixtures::*;
use crate::*;
use serde_json::json;

#[test]
fn constructors_and_deserialization_reject_bad_metadata() {
    for text in ["", " ", " padded", "padded ", "a\nb", "a\0b"] {
        assert!(AgentDescriptor::new(text, "1", None).is_err());
        assert!(AgentDescriptor::new("agent", text, None).is_err());
        assert!(ModelMetadata::new(text, "model", "1").is_err());
        assert!(ModelMetadata::new("provider", text, "1").is_err());
        assert!(ModelMetadata::new("provider", "model", text).is_err());
    }
    for pointer in [
        "/agent/name",
        "/agent/version",
        "/identity/id",
        "/identity/organization",
        "/identity/team",
        "/identity/owner",
        "/identity/purpose",
        "/identity/runtime",
        "/identity/environment",
        "/identity/device",
        "/requires/0/environment",
    ] {
        let mut value = input();
        *value.pointer_mut(pointer).unwrap() = json!("");
        assert!(
            serde_json::from_value::<AgentManifest>(value).is_err(),
            "{pointer}"
        );
    }
    assert!(AgentDescriptor::new("a".repeat(513), "1", None).is_err());
    for name in ["*", ".", "..", "agent/name"] {
        assert!(AgentDescriptor::new(name, "1", None).is_err());
    }
}
