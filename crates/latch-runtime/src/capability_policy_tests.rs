use super::gate_test_support::{UnreachableAdapter, engine, setup};
use super::*;
use latch_policy::Effect;

#[test]
fn changed_policy_never_inherits_capability_authorization() {
    for effect in [Effect::Ask, Effect::Deny] {
        let (mut gate, action) = setup();
        let token = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        gate.policies = engine(effect);
        assert!(matches!(
            gate.execute_with_capability(&token, &action, 21, &mut UnreachableAdapter),
            Err(CapabilityExecutionError::Execution(ExecutionError::Policy(
                _
            )))
        ));
        assert!(gate.capabilities().next().unwrap().consumed);
    }
    let (mut gate, action) = setup();
    gate.policies = engine(Effect::Ask);
    gate.approve_once(&action, "approver".into(), 20, 40)
        .unwrap();
    let token = gate
        .issue_capability(&action, "issuer".into(), 21, 40)
        .unwrap();
    gate.policies = engine(Effect::Deny);
    assert!(matches!(
        gate.issue_capability(&action, "issuer".into(), 22, 40),
        Err(CapabilityError::Policy(_))
    ));
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 22, &mut UnreachableAdapter),
        Err(CapabilityExecutionError::Execution(ExecutionError::Policy(
            _
        )))
    ));
}
