use crate::scenarios;
use crate::workflow_test_fixtures::*;
use latch_policy::{MAX_POLICY_DOCUMENT_BYTES, PolicyDocument};
use serde_json::json;

#[test]
fn scenario_dto_rejects_invalid_untrusted_fixtures() {
    let engine =
        PolicyDocument::from_json(policy_value(json!({"kind":"allow"})).to_string().as_bytes())
            .unwrap()
            .into_engine();
    for (pointer, replacement) in [
        ("/version", json!(2)),
        ("/scenarios", json!([])),
        ("/scenarios/0/id", json!("\u{1b}[2J")),
        ("/scenarios/0/id", json!("")),
        ("/scenarios/0/expected", json!("success")),
        ("/scenarios/0/session/id", json!("")),
        ("/scenarios/0/session/policies", json!([])),
        (
            "/scenarios/0/session/identity/id",
            json!("agent://other/coder"),
        ),
        ("/scenarios/0/session/identity/trust_level", json!(101)),
        ("/scenarios/0/session/identity/expires_at", json!(10)),
        ("/scenarios/0/session/identity/created_at", json!(11)),
        ("/scenarios/0/action/risk", json!(101)),
        ("/scenarios/0/action/name", json!("not-an-action")),
        ("/scenarios/0/action/resource", json!("")),
        ("/scenarios/0/action/reversibility", json!("safe")),
    ] {
        let mut scenario = scenario_value();
        *scenario.pointer_mut(pointer).unwrap() = replacement;
        let mut output = Vec::new();
        assert!(
            scenarios::test(&engine, scenario.to_string().as_bytes(), &mut output).is_err(),
            "{pointer}"
        );
        assert!(output.is_empty());
    }
    for pointer in [
        "",
        "/scenarios/0",
        "/scenarios/0/session",
        "/scenarios/0/session/identity",
        "/scenarios/0/action",
    ] {
        let mut scenario = scenario_value();
        scenario
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("trusted".into(), json!(true));
        assert!(
            scenarios::test(&engine, scenario.to_string().as_bytes(), &mut Vec::new()).is_err()
        );
    }
    let mut duplicate = scenario_value();
    let scenario = duplicate["scenarios"][0].clone();
    duplicate["scenarios"]
        .as_array_mut()
        .unwrap()
        .push(scenario.clone());
    assert!(
        scenarios::test(&engine, duplicate.to_string().as_bytes(), &mut Vec::new())
            .unwrap_err()
            .contains("DuplicateId")
    );
    duplicate["scenarios"] = json!(vec![scenario; 1025]);
    assert!(scenarios::test(&engine, duplicate.to_string().as_bytes(), &mut Vec::new()).is_err());
    for bytes in [
        b"{}".to_vec(),
        b"{\"version\":1,\"version\":1,\"scenarios\":[]}".to_vec(),
        b"version: 1\nscenarios: []".to_vec(),
        vec![b' '; MAX_POLICY_DOCUMENT_BYTES + 1],
    ] {
        assert!(scenarios::test(&engine, &bytes, &mut Vec::new()).is_err());
    }
}
