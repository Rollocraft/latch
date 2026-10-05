//! Capability tokens: issuance, binding, single use, revocation and audit.

use super::support::*;
use latch_core::*;
use latch_policy::*;
use latch_runtime::gate::*;

#[test]
fn capability_is_opaque_exact_and_single_use() {
    let (mut gate, action) = setup(Effect::Allow, 10);
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    assert_eq!(format!("{token:?}"), "CapabilityToken([REDACTED])");
    let imported = CapabilityToken::import_secret(&token.export_secret()).unwrap();
    assert_eq!(token, imported);
    let record = gate.capabilities().next().unwrap();
    assert_eq!(record.action, action);
    assert_eq!(record.organization, "acme");
    assert_eq!(record.session, "s1");
    assert_eq!(record.owner, "david");
    assert_eq!(record.issuer, "issuer");
    assert_eq!((record.issued_at, record.expires_at), (20, 40));
    let mut adapter = Adapter::default();
    for field in 0..9 {
        let mut changed = action.clone();
        match field {
            0 => changed.id.push('2'),
            1 => changed.actor.push('2'),
            2 => changed.session_id.push('2'),
            3 => changed.name = "file.read".into(),
            4 => changed.resource.push('2'),
            5 => changed.environment.push('2'),
            6 => changed.arguments.push("--force".into()),
            7 => changed.reversibility = Reversibility::Irreversible,
            _ => changed.risk = RiskScore::new(16).unwrap(),
        }
        assert!(matches!(
            gate.execute_with_capability(&token, &changed, 21, &mut adapter),
            Err(CapabilityExecutionError::Capability(
                CapabilityError::ScopeMismatch
            ))
        ));
    }
    assert_eq!(adapter.estimates.get(), 0);
    gate.execute_with_capability(&imported, &action, 21, &mut adapter)
        .unwrap();
    assert!(gate.capabilities().next().unwrap().consumed);
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 22, &mut adapter),
        Err(CapabilityExecutionError::Capability(
            CapabilityError::Consumed
        ))
    ));
    assert!(matches!(
        gate.execute(&action, 22, &mut adapter),
        Err(ExecutionError::AlreadyAttempted)
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn capability_issuance_requires_current_permission_and_valid_bounds() {
    for effect in [Effect::Deny, Effect::Ask] {
        let (mut gate, action) = setup(effect, 10);
        assert!(matches!(
            gate.issue_capability(&action, "issuer".into(), 20, 40),
            Err(CapabilityError::Policy(_))
        ));
        assert_eq!(gate.capabilities().count(), 0);
    }
    let (mut gate, action) = setup(Effect::Allow, 10);
    for issuer in ["", " ", "issuer\n"] {
        assert!(matches!(
            gate.issue_capability(&action, issuer.into(), 20, 40),
            Err(CapabilityError::InvalidIssuer)
        ));
    }
    for (now, expires_at) in [
        (9, 40),
        (20, 20),
        (20, 19),
        (20, 101),
        (100, 101),
        (20, u64::MAX),
    ] {
        assert!(matches!(
            gate.issue_capability(&action, "issuer".into(), now, expires_at),
            Err(CapabilityError::InvalidExpiration)
        ));
    }
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 100)
        .unwrap();
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 19, &mut adapter),
        Err(CapabilityExecutionError::Capability(
            CapabilityError::NotYetValid
        ))
    ));
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 100, &mut adapter),
        Err(CapabilityExecutionError::Capability(
            CapabilityError::Expired
        ))
    ));
    assert_eq!(adapter.estimates.get(), 0);
}

#[test]
fn capabilities_require_live_real_approvals_at_execution() {
    for revoke in [false, true] {
        let (mut gate, action) = setup(Effect::Ask, 10);
        gate.approve_once(&action, "approver".into(), 20, 30)
            .unwrap();
        let token = gate
            .issue_capability(&action, "issuer".into(), 21, 40)
            .unwrap();
        if revoke {
            assert!(gate.revoke_approval(&action.id));
        }
        let mut adapter = Adapter::default();
        assert!(matches!(
            gate.execute_with_capability(
                &token,
                &action,
                if revoke { 22 } else { 30 },
                &mut adapter
            ),
            Err(CapabilityExecutionError::Execution(ExecutionError::Policy(
                _
            )))
        ));
        assert!(gate.capabilities().next().unwrap().consumed);
        assert_eq!(adapter.estimates.get(), 0);
        assert!(
            gate.approve_once(&action, "approver".into(), 31, 40)
                .is_err()
        );
    }
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "approver".into(), 20, 30)
        .unwrap();
    let mut changed = action.clone();
    changed.arguments.push("changed".into());
    assert!(matches!(
        gate.issue_capability(&changed, "issuer".into(), 21, 40),
        Err(CapabilityError::Policy(_))
    ));
    let token = gate
        .issue_capability(&action, "issuer".into(), 21, 40)
        .unwrap();
    let mut adapter = Adapter::default();
    gate.execute_with_capability(&token, &action, 22, &mut adapter)
        .unwrap();
    assert!(gate.approvals().next().unwrap().consumed);
    assert_eq!(adapter.calls, 1);
}

#[test]
fn capability_revocation_and_transitions_are_permanent() {
    for state in [
        SessionState::Frozen,
        SessionState::Failed,
        SessionState::Committed,
        SessionState::RolledBack,
    ] {
        let (mut gate, action) = setup(Effect::Allow, 10);
        let token = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        assert!(gate.transition(SessionState::Active, 21).is_err());
        assert!(!gate.capabilities().next().unwrap().revoked);
        gate.transition(state, 21).unwrap();
        if state == SessionState::Frozen {
            gate.transition(SessionState::Active, 22).unwrap();
        }
        assert!(gate.capabilities().next().unwrap().revoked);
        assert!(matches!(
            gate.execute_with_capability(&token, &action, 23, &mut Adapter::default()),
            Err(CapabilityExecutionError::Capability(
                CapabilityError::Revoked
            ))
        ));
        assert!(
            gate.audit()
                .events()
                .iter()
                .any(|event| event.action == "runtime.capability.revoke")
        );
    }
    let (mut gate, action) = setup(Effect::Allow, 10);
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    let id = gate.capabilities().next().unwrap().id;
    assert!(gate.revoke_capability(id, 21).unwrap());
    assert!(!gate.revoke_capability(id, 22).unwrap());
    assert!(!gate.revoke_capability(u64::MAX, 22).unwrap());
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 23, &mut Adapter::default()),
        Err(CapabilityExecutionError::Capability(
            CapabilityError::Revoked
        ))
    ));
}

#[test]
fn capability_failures_consume_action_and_all_sibling_grants() {
    for failure in 0..3 {
        let (mut gate, action) = setup(Effect::Allow, if failure == 0 { 0 } else { 1 });
        let token = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        let sibling = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        assert_ne!(token, sibling);
        let mut adapter = Adapter {
            fail: failure == 1,
            ..Default::default()
        };
        if failure == 2 {
            assert!(matches!(
                gate.execute_with_capability(&token, &action, 21, &mut PreparationFailure),
                Err(CapabilityExecutionError::Execution(
                    ExecutionError::Adapter(_)
                ))
            ));
        } else {
            assert!(
                gate.execute_with_capability(&token, &action, 21, &mut adapter)
                    .is_err()
            );
        }
        assert!(gate.capabilities().all(|record| record.consumed));
        assert!(matches!(
            gate.execute_with_capability(&sibling, &action, 22, &mut adapter),
            Err(CapabilityExecutionError::Capability(
                CapabilityError::Consumed
            ))
        ));
        assert!(matches!(
            gate.execute(&action, 22, &mut adapter),
            Err(ExecutionError::AlreadyAttempted)
        ));
        assert!(matches!(
            gate.issue_capability(&action, "issuer".into(), 22, 40),
            Err(CapabilityError::AlreadyAttempted)
        ));
        assert_eq!(adapter.calls, usize::from(failure == 1));
    }
}

#[test]
fn capability_budget_is_shared_with_normal_execution() {
    let (mut gate, action) = setup(Effect::Allow, 1);
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    let mut other = action.clone();
    other.id = "other".into();
    let mut adapter = Adapter::default();
    gate.execute(&other, 21, &mut adapter).unwrap();
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 22, &mut adapter),
        Err(CapabilityExecutionError::Execution(ExecutionError::Budget(
            _
        )))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn unknown_and_cross_gate_tokens_fail_without_adapter_calls() {
    let (mut gate, action) = setup(Effect::Allow, 1);
    let (mut other, _) = setup(Effect::Allow, 1);
    let token = other
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 21, &mut adapter),
        Err(CapabilityExecutionError::Capability(
            CapabilityError::Unknown
        ))
    ));
    assert_eq!(adapter.estimates.get(), 0);
    gate.execute(&action, 22, &mut adapter).unwrap();
}
