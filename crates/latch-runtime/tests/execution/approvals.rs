//! Approval grants: exactness, expiry, consumption, revocation and audit.

use super::support::*;
use latch_core::*;
use latch_policy::*;
use latch_runtime::gate::*;

#[test]
fn approval_is_exact_expiring_and_consumed_once() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    let mut adapter = Adapter::default();
    gate.approve_once(&action, "david".into(), 20, 30).unwrap();
    let mut changed = action.clone();
    changed.arguments.push("--force".into());
    assert!(matches!(
        gate.execute(&changed, 21, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    assert!(matches!(
        gate.execute(&action, 19, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    gate.execute(&action, 21, &mut adapter).unwrap();
    assert!(gate.approvals().next().unwrap().consumed);
    assert!(gate.execute(&action, 22, &mut adapter).is_err());
    assert_eq!(adapter.calls, 1);

    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "david".into(), 20, 30).unwrap();
    assert!(matches!(
        gate.execute(&action, 30, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn approval_never_overrides_deny_or_budget() {
    let (mut gate, action) = setup(Effect::Deny, 10);
    assert_eq!(
        gate.approve_once(&action, "david".into(), 20, 30),
        Err(ApprovalError::NotPending)
    );
    let (mut gate, action) = setup(Effect::Ask, 0);
    gate.approve_once(&action, "david".into(), 20, 30).unwrap();
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute(&action, 21, &mut adapter),
        Err(ExecutionError::Budget(_))
    ));
    assert_eq!(adapter.calls, 0);
    assert!(!gate.approvals().next().unwrap().consumed);
}

#[test]
fn approval_cannot_outlive_identity_or_hide_approver() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    assert_eq!(
        gate.approve_once(&action, "".into(), 20, 30),
        Err(ApprovalError::InvalidApprover)
    );
    assert_eq!(
        gate.approve_once(&action, "david".into(), 20, 101),
        Err(ApprovalError::InvalidExpiration)
    );
    assert_eq!(
        gate.approve_once(&action, "david".into(), 20, 20),
        Err(ApprovalError::InvalidExpiration)
    );
}
#[test]
fn freeze_revokes_permissions_even_after_resume() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "david".into(), 20, 40).unwrap();
    gate.transition(SessionState::Frozen, 21).unwrap();
    gate.transition(SessionState::Active, 22).unwrap();
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute(&action, 23, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    assert_eq!(adapter.calls, 0);
    let record = gate.approvals().next().unwrap();
    assert!(record.revoked);
    assert_eq!(record.approver, "david");
}

#[test]
fn explicit_revocation_is_idempotent_and_preserves_record() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "david".into(), 20, 40).unwrap();
    assert!(gate.revoke_approval(&action.id));
    assert!(!gate.revoke_approval(&action.id));
    assert!(!gate.revoke_approval("unknown"));
    let mut adapter = Adapter::default();
    assert!(gate.execute(&action, 21, &mut adapter).is_err());
    assert_eq!(adapter.calls, 0);
    assert_eq!(gate.approvals().count(), 1);
}

#[test]
fn rejected_transition_does_not_revoke_approval() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "david".into(), 20, 40).unwrap();
    assert!(gate.transition(SessionState::Active, 21).is_err());
    let mut adapter = Adapter::default();
    gate.execute(&action, 22, &mut adapter).unwrap();
    assert_eq!(adapter.calls, 1);
}
#[test]
fn expired_approval_can_be_renewed_without_erasing_history() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "first".into(), 20, 30).unwrap();
    assert_eq!(
        gate.approve_once(&action, "second".into(), 25, 40),
        Err(ApprovalError::AlreadyApproved)
    );
    gate.approve_once(&action, "second".into(), 30, 40).unwrap();
    let records: Vec<_> = gate.approvals().collect();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].approver, "first");
    assert_eq!(records[0].expires_at, 30);
    assert_eq!(records[1].approver, "second");
    let mut adapter = Adapter::default();
    gate.execute(&action, 31, &mut adapter).unwrap();
    assert_eq!(adapter.calls, 1);
}

#[test]
fn renewed_permission_requires_same_action_after_revocation() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "first".into(), 20, 40).unwrap();
    gate.revoke_approval(&action.id);
    let mut changed = action.clone();
    changed.resource = "project/other".into();
    assert_eq!(
        gate.approve_once(&changed, "second".into(), 21, 40),
        Err(ApprovalError::ActionChanged)
    );
    gate.approve_once(&action, "second".into(), 21, 40).unwrap();
    assert!(gate.approvals().next().unwrap().revoked);
    let mut adapter = Adapter::default();
    gate.execute(&action, 22, &mut adapter).unwrap();
    assert_eq!(
        gate.approve_once(&action, "third".into(), 23, 40),
        Err(ApprovalError::NotPending)
    );
}
#[test]
fn unauditable_approval_grant_is_refused_fail_closed() {
    let (mut gate, action) = setup_with_audit(Effect::Ask, 10, FailingAudit { remaining: 0 });
    assert_eq!(
        gate.approve_once(&action, "david".into(), 20, 30),
        Err(ApprovalError::ApproveAuditFailed)
    );
    assert_eq!(gate.approvals().count(), 0);
    let mut adapter = Adapter::default();
    // With the sink dead even the ASK decision cannot be recorded: execution
    // fails before any effect, and no approval exists to authorize it anyway.
    assert!(matches!(
        gate.execute(&action, 21, &mut adapter),
        Err(ExecutionError::AuditBeforeExecution("disk full"))
    ));
    assert_eq!(adapter.calls, 0);
}

#[test]
fn approval_grant_is_audited_with_attribution() {
    let (mut gate, action) = setup(Effect::Ask, 10);
    gate.approve_once(&action, "david".into(), 20, 30).unwrap();
    let grants: Vec<_> = gate
        .audit()
        .events()
        .iter()
        .filter(|event| event.action.starts_with("runtime.approval."))
        .collect();
    assert_eq!(grants.len(), 1, "grant recorded exactly once: {:?}", grants);
    assert_eq!(grants[0].action, "runtime.approval.grant");
    assert_eq!(grants[0].resource, "approval/david");
    assert_eq!(grants[0].owner, "david");
    assert_eq!(grants[0].session, "s1");
    gate.revoke_approval(&action.id);
    let all: Vec<_> = gate
        .audit()
        .events()
        .iter()
        .filter(|event| event.action.starts_with("runtime.approval."))
        .collect();
    assert_eq!(all.len(), 1, "revocation keeps attribution, adds no event");
}
