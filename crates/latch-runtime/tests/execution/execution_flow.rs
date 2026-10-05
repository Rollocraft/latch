//! The execution path itself: policy, budget, replay, audit and attribution.

use super::support::*;
use latch_core::*;
use latch_policy::*;
use latch_runtime::gate::*;

#[test]
fn denied_and_unapproved_actions_never_reach_adapter() {
    for effect in [Effect::Deny, Effect::Ask] {
        let (mut gate, action) = setup(effect, 10);
        let mut adapter = Adapter::default();
        assert!(matches!(
            gate.execute(&action, 20, &mut adapter),
            Err(ExecutionError::Policy(_))
        ));
        assert_eq!(adapter.calls, 0);
        assert_eq!(adapter.estimates.get(), 0);
    }
}

#[test]
fn budget_is_charged_before_execution_and_replay_is_rejected() {
    let (mut gate, mut action) = setup(Effect::Allow, 1);
    let mut adapter = Adapter::default();
    gate.execute(&action, 20, &mut adapter).unwrap();
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::AlreadyAttempted)
    ));
    action.id = "a2".into();
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::Budget(_))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn ambiguous_adapter_failure_keeps_charge_and_consumes_action_id() {
    let (mut gate, mut action) = setup(Effect::Allow, 1);
    let mut adapter = Adapter {
        fail: true,
        ..Default::default()
    };
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::Adapter(()))
    ));
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::AlreadyAttempted)
    ));
    action.id = "a2".into();
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::Budget(_))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn frozen_and_expired_sessions_cannot_execute() {
    let (mut gate, action) = setup(Effect::Allow, 10);
    let mut adapter = Adapter::default();
    gate.transition(SessionState::Frozen, 20).unwrap();
    assert!(matches!(
        gate.execute(&action, 21, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    gate.transition(SessionState::Active, 22).unwrap();
    assert!(matches!(
        gate.execute(&action, 100, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    assert_eq!(adapter.calls, 0);
}

#[test]
fn start_audit_failure_prevents_effects() {
    let (mut gate, action) = setup_with_audit(Effect::Allow, 10, FailingAudit { remaining: 0 });
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::AuditBeforeExecution("disk full"))
    ));
    assert_eq!(adapter.calls, 0);
}

#[test]
fn completion_audit_failure_does_not_allow_replay() {
    let (mut gate, action) = setup_with_audit(Effect::Allow, 10, FailingAudit { remaining: 1 });
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::AuditAfterExecution("disk full"))
    ));
    assert!(matches!(
        gate.execute(&action, 21, &mut adapter),
        Err(ExecutionError::AuditBeforeExecution("disk full"))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn budget_and_replay_rejections_are_audited() {
    let (mut gate, mut action) = setup(Effect::Allow, 1);
    let mut adapter = Adapter::default();
    gate.execute(&action, 20, &mut adapter).unwrap();
    assert!(gate.execute(&action, 21, &mut adapter).is_err());
    action.id = "second".into();
    assert!(gate.execute(&action, 22, &mut adapter).is_err());
    let results: Vec<_> = gate.audit().events().iter().map(|e| e.result).collect();
    assert_eq!(
        results,
        vec![
            latch_audit::EventResult::Started,
            latch_audit::EventResult::Succeeded,
            latch_audit::EventResult::ReplayRejected,
            latch_audit::EventResult::BudgetRejected
        ]
    );
    assert_eq!(adapter.calls, 1);
}

#[test]
fn execution_records_start_and_result_with_session_attribution() {
    let (mut gate, action) = setup(Effect::Allow, 10);
    gate.execute(&action, 20, &mut Adapter::default()).unwrap();
    let events = gate.audit().events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].result, latch_audit::EventResult::Started);
    assert_eq!(events[1].result, latch_audit::EventResult::Succeeded);
    assert_eq!(events[1].owner, "david");
    assert_eq!(events[1].action_id, action.id);
}

#[test]
fn normal_execution_consumes_preissued_capabilities() {
    let (mut gate, action) = setup(Effect::Allow, 10);
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    let mut adapter = Adapter::default();
    gate.execute(&action, 21, &mut adapter).unwrap();
    assert!(gate.capabilities().next().unwrap().consumed);
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 22, &mut adapter),
        Err(CapabilityExecutionError::Capability(
            CapabilityError::Consumed
        ))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn capability_adapter_failure_keeps_budget_charge() {
    let (mut gate, action) = setup(Effect::Allow, 1);
    let token = gate
        .issue_capability(&action, "issuer".into(), 20, 40)
        .unwrap();
    let mut adapter = Adapter {
        fail: true,
        ..Default::default()
    };
    assert!(matches!(
        gate.execute_with_capability(&token, &action, 21, &mut adapter),
        Err(CapabilityExecutionError::Execution(
            ExecutionError::Adapter(())
        ))
    ));
    let mut other = action.clone();
    other.id = "other".into();
    let other_token = gate
        .issue_capability(&other, "issuer".into(), 22, 40)
        .unwrap();
    assert!(matches!(
        gate.execute_with_capability(&other_token, &other, 23, &mut adapter),
        Err(CapabilityExecutionError::Execution(ExecutionError::Budget(
            _
        )))
    ));
    assert_eq!(adapter.calls, 1);
}

#[test]
fn preparation_failure_is_audited_without_consuming_budget_or_action() {
    let (mut gate, action) = setup(Effect::Allow, 1);
    assert!(matches!(
        gate.execute(&action, 20, &mut PreparationFailure),
        Err(ExecutionError::Adapter("unknown budget unit"))
    ));
    assert_eq!(
        gate.audit().events()[0].result,
        latch_audit::EventResult::PreparationFailed
    );
    let mut adapter = Adapter::default();
    gate.execute(&action, 21, &mut adapter).unwrap();
    assert_eq!(adapter.calls, 1);
}

#[test]
fn forged_attribution_is_denied_and_logged_under_real_session() {
    let (mut gate, mut action) = setup(Effect::Allow, 1);
    action.actor = "agent://other/coder".into();
    action.session_id = "other-session".into();
    action.environment = "production".into();
    let mut adapter = Adapter::default();
    assert!(matches!(
        gate.execute(&action, 20, &mut adapter),
        Err(ExecutionError::Policy(_))
    ));
    let event = &gate.audit().events()[0];
    assert_eq!(event.result, latch_audit::EventResult::Denied);
    assert_eq!(event.organization, "acme");
    assert_eq!(event.agent, "agent://acme/coder");
    assert_eq!(event.session, "s1");
    assert_eq!(event.environment, "dev");
    assert_eq!(adapter.calls, 0);
    assert_eq!(adapter.estimates.get(), 0);
}
#[test]
fn execution_audit_survives_gate_shutdown() {
    let path = std::env::temp_dir().join(format!(
        "latch-runtime-audit-{}-{}.bin",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let sink = latch_audit::FileAudit::create(&path).unwrap();
    let (mut gate, mut action) = setup_with_audit(Effect::Allow, 1, sink);
    let mut adapter = Adapter::default();
    gate.execute(&action, 20, &mut adapter).unwrap();
    action.id = "over-budget".into();
    assert!(matches!(
        gate.execute(&action, 21, &mut adapter),
        Err(ExecutionError::Budget(_))
    ));
    drop(gate);
    let events = latch_audit::FileAudit::read(&path, 8192).unwrap();
    assert_eq!(
        events.iter().map(|e| e.result).collect::<Vec<_>>(),
        vec![
            latch_audit::EventResult::Started,
            latch_audit::EventResult::Succeeded,
            latch_audit::EventResult::BudgetRejected
        ]
    );
    assert!(
        events
            .iter()
            .all(|e| e.organization == "acme" && e.session == "s1" && e.owner == "david")
    );
    assert_eq!(events[2].action_id, "over-budget");
    assert_eq!(adapter.calls, 1);
    std::fs::remove_file(path).unwrap();
}
