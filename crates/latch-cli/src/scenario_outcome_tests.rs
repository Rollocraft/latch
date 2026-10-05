use crate::workflow_test_fixtures::*;
use serde_json::json;

#[test]
fn policy_tests_run_the_harness_and_report_mismatches() {
    let files = Files::new();
    let policy = files.write(
        "policy.json",
        policy_value(json!({"kind":"allow"})).to_string(),
    );
    let scenario = files.write("scenario.json", scenario_value().to_string());
    let (result, output) = call(&["policy".as_ref(), "test".as_ref(), &policy, &scenario]);
    assert!(result.is_ok(), "{result:?}");
    assert!(output.contains("PASS \"example\": Allow"));
    assert!(output.contains("1 passed; 0 failed"));
    let mut value = scenario_value();
    value["scenarios"][0]["expected"] = json!("deny");
    let scenario = files.write("scenario.json", value.to_string());
    let (result, output) = call(&["policy".as_ref(), "test".as_ref(), &policy, &scenario]);
    assert!(result.unwrap_err().contains("1 policy scenario(s) failed"));
    assert!(output.contains("Outcome { expected: Deny, actual: Allow }"));
    assert!(output.contains("0 passed; 1 failed"));
}

#[test]
fn policy_tests_cover_all_outcomes_and_argument_matching() {
    let files = Files::new();
    for (effect, expected) in [
        (json!({"kind":"allow"}), "allow"),
        (json!({"kind":"deny"}), "deny"),
        (json!({"kind":"ask"}), "approval_required"),
        (
            json!({"kind":"limit","budget":"requests","maximum":2}),
            "limited",
        ),
    ] {
        let mut policy = policy_value(effect);
        let mut allow = policy["policies"][0]["rules"][0].clone();
        allow["id"] = json!("allow");
        allow["effect"] = json!({"kind":"allow"});
        policy["policies"][0]["rules"]
            .as_array_mut()
            .unwrap()
            .push(allow);
        let mut scenario = scenario_value();
        scenario["scenarios"][0]["expected"] = json!(expected);
        let policy = files.write("policy.json", policy.to_string());
        let scenario = files.write("scenario.json", scenario.to_string());
        let (result, _) = call(&["policy".as_ref(), "test".as_ref(), &policy, &scenario]);
        assert!(result.is_ok(), "{expected}: {result:?}");
    }
    let mut policy = policy_value(json!({"kind":"allow"}));
    policy["policies"][0]["rules"][0]["arguments"] = json!({"kind":"flag","value":"--check"});
    let policy = files.write("policy.json", policy.to_string());
    let mut scenario = scenario_value();
    scenario["scenarios"][0]["action"]["arguments"] = json!(["--check"]);
    let scenario = files.write("scenario.json", scenario.to_string());
    assert!(
        call(&["policy".as_ref(), "test".as_ref(), &policy, &scenario])
            .0
            .is_ok()
    );
}
