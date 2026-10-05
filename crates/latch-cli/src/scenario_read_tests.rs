use crate::workflow_test_fixtures::*;
use latch_policy::MAX_POLICY_DOCUMENT_BYTES;
use serde_json::json;

#[test]
fn scenario_command_bounds_reads_and_validates_before_reporting_passes() {
    let files = Files::new();
    let policy = files.write("policy.yaml", "version: 1\npolicies: []\n");
    let oversize = files.write("scenarios.json", vec![b' '; MAX_POLICY_DOCUMENT_BYTES + 1]);
    assert!(
        call(&["policy".as_ref(), "test".as_ref(), &policy, &oversize])
            .0
            .unwrap_err()
            .contains("byte limit")
    );
    let mut scenario = scenario_value();
    let mut invalid = scenario["scenarios"][0].clone();
    invalid["id"] = json!("second");
    invalid["action"]["risk"] = json!(200);
    scenario["scenarios"].as_array_mut().unwrap().push(invalid);
    let scenario = files.write("scenarios.json", scenario.to_string());
    let (result, output) = call(&["policy".as_ref(), "test".as_ref(), &policy, &scenario]);
    assert!(result.is_err());
    assert!(output.is_empty());
}
