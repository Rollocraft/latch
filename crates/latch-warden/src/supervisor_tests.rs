use crate::supervisor_test_backends::*;
use crate::*;

#[test]
fn deterministic_lifecycle_and_idempotent_cleanup() {
    let mut supervisor = Supervisor::with_external_authorization_and_audit();
    let authorization = authorization("acme");
    let calls = Calls::default();
    let (handle, probe) = register(&mut supervisor, &authorization, "session", &calls);
    assert_eq!(handle.organization(), "acme");
    assert_eq!(handle.session_id(), "session");
    assert_eq!(
        supervisor.status(&authorization, &handle).unwrap().state,
        SandboxState::Planned
    );
    assert!(supervisor.start(&authorization, &handle).is_err());
    supervisor.prepare(&authorization, &handle).unwrap();
    let running = supervisor.start(&authorization, &handle).unwrap();
    assert_eq!(running.state, SandboxState::Running);
    assert_eq!(
        running.enforcement,
        EnforcementStatus::BackendReported {
            assurance: AssuranceLevel::CooperativeMediation,
        }
    );
    supervisor.freeze(&authorization, &handle).unwrap();
    supervisor.freeze(&authorization, &handle).unwrap();
    let terminated = supervisor.terminate(&authorization, &handle).unwrap();
    assert_eq!(terminated.state, SandboxState::Terminated);
    assert_eq!(terminated.enforcement, EnforcementStatus::NotEstablished);
    supervisor.terminate(&authorization, &handle).unwrap();
    assert!(!probe.allocated.get());
    assert_eq!(
        calls
            .borrow()
            .iter()
            .map(|(_, operation)| *operation)
            .collect::<Vec<_>>(),
        [
            Operation::Plan,
            Operation::Plan,
            Operation::Prepare,
            Operation::Plan,
            Operation::Start,
            Operation::Plan,
            Operation::Freeze,
            Operation::Terminate,
        ]
    );
    assert!(supervisor.start(&authorization, &handle).is_err());
    assert!(supervisor.freeze(&authorization, &handle).is_err());
}

#[test]
fn cleanup_is_available_from_every_registered_nonterminal_state() {
    for steps in 0..=3 {
        let mut supervisor = Supervisor::with_external_authorization_and_audit();
        let authorization = authorization("acme");
        let calls = Calls::default();
        let (handle, probe) = register(&mut supervisor, &authorization, "session", &calls);
        if steps >= 1 {
            supervisor.prepare(&authorization, &handle).unwrap();
        }
        if steps >= 2 {
            supervisor.start(&authorization, &handle).unwrap();
        }
        if steps >= 3 {
            supervisor.freeze(&authorization, &handle).unwrap();
        }
        let report = supervisor.kill_organization(&authorization);
        assert!(report.all_terminated());
        assert!(!probe.allocated.get());
        assert_eq!(calls.borrow().last().unwrap().1, Operation::Terminate);
    }
}
