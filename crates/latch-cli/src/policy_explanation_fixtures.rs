use crate::inspection_test_support::{Fixture, invoke};
use serde_json::{Value, json};
use std::ffi::OsString;

pub(crate) fn policy(fixture: &Fixture, effects: &[Value]) -> OsString {
    let rules: Vec<_> = effects
        .iter()
        .enumerate()
        .map(|(i, effect)| {
            json!({
                "id": format!("rule-{i}"), "description": "hypothetical rule",
                "action": {"kind":"exact","value":"file.read"},
                "resource": {"kind":"exact","value":"example"},
                "environment": {"kind":"any"}, "actor": {"kind":"any"}, "effect":effect
            })
        })
        .collect();
    let path = fixture.0.join("policy.json");
    std::fs::write(
        &path,
        json!({"version":1,"policies":[{
            "id":"local", "level":"company", "rules":rules
        }]})
        .to_string(),
    )
    .unwrap();
    path.into_os_string()
}

pub(crate) fn explain(path: OsString, resource: Option<&str>) -> Result<String, String> {
    let mut args = vec!["policy".into(), "explain".into(), path, "file.read".into()];
    if let Some(resource) = resource {
        args.push(resource.into());
    }
    invoke(args)
}

pub(crate) fn limit() -> Value {
    json!({"kind":"limit","budget":"reads","maximum":4})
}
