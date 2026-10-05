use crate::{
    inspection_test_support::{Fixture, invoke},
    policy_explanation_fixtures::*,
};
use serde_json::json;

#[test]
fn explanations_report_all_outcomes_and_retained_limits() {
    let fixture = Fixture::new("explain");
    for (effects, outcome) in [
        (vec![json!({"kind":"allow"})], "ALLOW"),
        (vec![json!({"kind":"deny"})], "DENY"),
        (vec![json!({"kind":"ask"})], "ASK"),
        (vec![json!({"kind":"allow"}), limit()], "LIMIT"),
        (vec![json!({"kind":"ask"}), limit()], "ASK"),
    ] {
        let path = policy(&fixture, &effects);
        let before = std::fs::read(&path).unwrap();
        let output = explain(path.clone(), Some("example")).unwrap();
        assert!(
            output.contains(&format!("final outcome: {outcome}")),
            "{output}"
        );
        assert!(output.contains("policy=\"local\" rule=\"rule-0\""));
        assert!(output.contains("not runtime enforcement or audit history"));
        if effects.len() == 2 {
            assert!(output.contains("budget=\"reads\" maximum=4"));
        }
        assert_eq!(std::fs::read(path).unwrap(), before);
    }
}

#[test]
fn omitted_resource_is_literal_and_empty_policy_denies() {
    let fixture = Fixture::new("defaults");
    let path = policy(&fixture, &[json!({"kind":"allow"})]);
    let output = explain(path.clone(), None).unwrap();
    assert!(output.contains("resource: \"unspecified\""));
    assert!(output.contains("matched rules: 0\nfinal outcome: DENY"));
    std::fs::write(&path, br#"{"version":1,"policies":[]}"#).unwrap();
    assert!(explain(path, None).unwrap().contains("reason: DefaultDeny"));
}

#[test]
fn invalid_explanation_inputs_fail_and_help_is_truthful() {
    let fixture = Fixture::new("invalid-explain");
    let path = policy(&fixture, &[]);
    for action in ["invalid", "file..read", "FILE.read", "file.read\n"] {
        let error = invoke(vec![
            "policy".into(),
            "explain".into(),
            path.clone(),
            action.into(),
        ])
        .unwrap_err();
        assert!(error.contains("action"));
        assert!(!error.contains('\n'));
    }
    assert!(explain(path.clone(), Some("\u{1b}[2J")).is_err());
    std::fs::write(&path, b"invalid JSON").unwrap();
    assert!(explain(path, None).unwrap_err().contains("invalid policy"));
    let help = invoke(vec!["policy".into(), "explain".into(), "--help".into()]).unwrap();
    assert!(help.contains("No event store exists"));
    assert!(help.contains("environment: offline; arguments: []"));
    assert!(invoke(vec!["policy".into(), "explain".into()]).is_err());
}
