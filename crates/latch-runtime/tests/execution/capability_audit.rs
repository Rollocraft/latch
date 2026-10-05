//! Capability audit: fail-closed issuance, pending metadata and public fields.

use super::support::*;
use latch_core::*;
use latch_policy::*;
use latch_runtime::gate::*;

#[test]
fn capability_audit_failures_fail_closed_and_preserve_consumption() {
    let (mut gate, action) = setup_with_audit(Effect::Allow, 10, FailingAudit { remaining: 0 });
    assert!(matches!(
        gate.issue_capability(&action, "issuer".into(), 20, 40),
        Err(CapabilityError::Audit("disk full"))
    ));
    assert!(gate.capabilities().all(|record| record.revoked));
    assert_eq!(gate.capabilities().count(), 1);
    for remaining in [1, 2] {
        let (mut gate, action) = setup_with_audit(Effect::Allow, 10, FailingAudit { remaining });
        let token = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        let mut adapter = Adapter::default();
        let result = gate.execute_with_capability(&token, &action, 21, &mut adapter);
        if remaining == 1 {
            assert!(matches!(
                result,
                Err(CapabilityExecutionError::Execution(
                    ExecutionError::AuditBeforeExecution("disk full")
                ))
            ));
        } else {
            assert!(matches!(
                result,
                Err(CapabilityExecutionError::Execution(
                    ExecutionError::AuditAfterExecution("disk full")
                ))
            ));
        }
        assert!(gate.capabilities().next().unwrap().consumed);
        assert!(
            gate.execute_with_capability(&token, &action, 22, &mut adapter)
                .is_err()
        );
        assert_eq!(adapter.calls, remaining - 1);
    }
}

#[test]
fn revocation_audit_failure_retains_pending_metadata_and_blocks_execution() {
    for transition in [false, true] {
        let (mut gate, action) = setup_with_audit(Effect::Allow, 10, RecoverableAudit::default());
        let token = gate
            .issue_capability(&action, "issuer".into(), 20, 40)
            .unwrap();
        let id = gate.capabilities().next().unwrap().id;
        gate.audit().failed.set(true);
        if transition {
            gate.transition(SessionState::Frozen, 21).unwrap();
            gate.transition(SessionState::Active, 22).unwrap();
        } else {
            assert!(matches!(
                gate.revoke_capability(id, 21),
                Err(CapabilityError::Audit("unavailable"))
            ));
        }
        assert!(gate.capabilities().next().unwrap().revoked);
        assert_eq!(gate.pending_capability_audit().count(), 1);
        let mut adapter = Adapter::default();
        assert!(
            gate.execute_with_capability(&token, &action, 23, &mut adapter)
                .is_err()
        );
        assert!(matches!(
            gate.execute(&action, 23, &mut adapter),
            Err(ExecutionError::AuditBeforeExecution("unavailable"))
        ));
        assert!(matches!(
            gate.issue_capability(&action, "issuer".into(), 23, 40),
            Err(CapabilityError::Audit("unavailable"))
        ));
        assert_eq!(adapter.estimates.get(), 0);
        gate.audit().failed.set(false);
        gate.flush_capability_audit().unwrap();
        assert_eq!(gate.pending_capability_audit().count(), 0);
        assert_eq!(
            gate.audit().events.last().unwrap().action,
            "runtime.capability.revoke"
        );
        assert!(matches!(
            gate.execute_with_capability(&token, &action, 24, &mut adapter),
            Err(CapabilityExecutionError::Capability(
                CapabilityError::Revoked
            ))
        ));
        gate.execute(&action, 24, &mut adapter).unwrap();
    }
}

#[test]
fn capability_audit_contains_only_public_metadata() {
    let (mut gate, mut action) = setup(Effect::Allow, 10);
    action.arguments.push("synthetic-private-argument".into());
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    let id = gate.capabilities().next().unwrap().id;
    gate.revoke_capability(id, 21).unwrap();
    let events = gate.audit().events();
    assert_eq!(events.len(), 2);
    for (event, operation, result, timestamp) in [
        (
            &events[0],
            "runtime.capability.issue",
            latch_audit::EventResult::Authorized,
            20,
        ),
        (
            &events[1],
            "runtime.capability.revoke",
            latch_audit::EventResult::Denied,
            21,
        ),
    ] {
        let mut expected =
            latch_audit::AuditEvent::for_action(gate.session(), &action, result, timestamp);
        expected.action = operation.into();
        expected.resource = format!("capability/{id}");
        assert_eq!(*event, expected);
    }
    let rendered = format!("{events:?}");
    assert!(!rendered.contains("synthetic-private-argument"));
    assert!(!rendered.contains(&format!("{:?}", token.export_secret())));
    assert!(
        !format!("{:?}", gate.capabilities().collect::<Vec<_>>())
            .contains(&format!("{:?}", token.export_secret()))
    );
}

#[test]
fn failed_issue_is_revoked_and_audit_can_recover() {
    let (mut gate, action) = setup_with_audit(Effect::Allow, 10, RecoverableAudit::default());
    gate.audit().failed.set(true);
    assert!(matches!(
        gate.issue_capability(&action, "issuer".into(), 20, 40),
        Err(CapabilityError::Audit("unavailable"))
    ));
    assert!(gate.capabilities().next().unwrap().revoked);
    assert_eq!(gate.pending_capability_audit().count(), 1);
    gate.audit().failed.set(false);
    let token = gate
        .issue_capability(&action, "issuer".into(), 21, 40)
        .unwrap();
    assert_eq!(gate.capabilities().count(), 2);
    assert_eq!(gate.audit().events[0].action, "runtime.capability.revoke");
    gate.execute_with_capability(&token, &action, 22, &mut Adapter::default())
        .unwrap();
}
