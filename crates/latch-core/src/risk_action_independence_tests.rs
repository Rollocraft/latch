use super::risk_test_support::*;
use super::*;

#[test]
fn calculation_does_not_consume_or_modify_caller_action_risk() {
    let mut action = crate::Action {
        id: "action-1".into(),
        actor: "agent://acme/coder".into(),
        session_id: "session-1".into(),
        name: "file.read".into(),
        resource: "file.txt".into(),
        environment: "development".into(),
        arguments: Vec::new(),
        reversibility: Reversibility::FullyReversible,
        risk: RiskScore::new(0).unwrap(),
    };
    let independently_supplied_factors = minimal(45);
    let first = calculate_risk(&independently_supplied_factors);
    assert_eq!(action.risk.value(), 0);
    action.risk = RiskScore::new(100).unwrap();
    assert_eq!(calculate_risk(&independently_supplied_factors), first);
    assert_eq!(action.risk.value(), 100);
    assert_eq!(first.score().value(), 45);
}
