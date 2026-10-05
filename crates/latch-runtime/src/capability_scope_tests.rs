use super::gate_test_support::{UnreachableAdapter, setup};
use super::*;

#[test]
fn server_scope_checks_all_attribution_fields() {
    for field in 0..3 {
        let (mut gate, action) = setup();
        let token = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        let record = gate.capabilities.get_mut(&token).unwrap();
        match field {
            0 => record.organization = "other".into(),
            1 => record.session = "other".into(),
            _ => record.owner = "other".into(),
        }
        assert!(matches!(
            gate.execute_with_capability(&token, &action, 21, &mut UnreachableAdapter),
            Err(CapabilityExecutionError::Capability(
                CapabilityError::ScopeMismatch
            ))
        ));
        assert!(!gate.capabilities().next().unwrap().consumed);
    }
}

#[test]
fn identifier_exhaustion_does_not_issue() {
    let (mut gate, action) = setup();
    gate.next_capability_id = u64::MAX;
    assert!(matches!(
        gate.issue_capability(&action, "issuer".into(), 20, 40),
        Err(CapabilityError::IdentifierExhausted)
    ));
    assert_eq!(gate.capabilities().count(), 0);
    assert!(gate.audit().events().is_empty());
}
