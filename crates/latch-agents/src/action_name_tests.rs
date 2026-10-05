use crate::*;
use serde_json::json;

#[test]
fn action_names_are_exact_and_core_compatible() {
    for name in [
        "",
        "git",
        "git.*",
        "git..push",
        "Git.push",
        "git.push ",
        "git.push\n",
        "git/push",
        "git.ÃƒÂ©",
    ] {
        assert!(ActionName::new(name).is_err(), "{name:?}");
        assert!(serde_json::from_value::<ActionName>(json!(name)).is_err());
    }
    assert!(ActionName::new("custom.api_v2.read").is_ok());
    assert!(ActionName::new(format!("file.{}", "a".repeat(256))).is_err());
}
