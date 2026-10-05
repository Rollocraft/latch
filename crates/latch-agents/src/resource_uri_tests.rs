use crate::*;
use serde_json::json;

#[test]
fn resources_reject_ambiguous_noncanonical_syntax() {
    for resource in [
        "",
        "/project",
        "file:///",
        "file://",
        "repo://",
        "repo:///main",
        "FILE:///project",
        "file:///project/",
        "file:///project//src",
        "file:///project/../other",
        "file:///project/./src",
        "file:///project/%61",
        "file:///project\\src",
        "file:///project/*",
        "file:///project/[ab]",
        "file:///project?x",
        "file:///project#x",
        "file:///project:stream",
        "file:///project/src.",
        "file:///project/a b",
        "file:///project/ÃƒÂ©",
        "file:///project/a\n",
        "repo://ACME/main",
        "repo://acme./main",
        "repo://user@host/main",
        "https://host:443/path",
    ] {
        assert!(ResourceId::new(resource).is_err(), "{resource:?}");
        assert!(serde_json::from_value::<ResourceId>(json!(resource)).is_err());
    }
    for resource in [
        "file:///project/src",
        "repo://acme/backend/main",
        "https://example.test",
        "custom+v1://tenant/object_1",
    ] {
        let id = ResourceId::new(resource).unwrap();
        assert_eq!(id.as_str(), resource);
        assert_eq!(id.scheme(), resource.split_once("://").unwrap().0);
    }
    assert!(ResourceId::new(format!("file:///{}", "a".repeat(2048))).is_err());
}
