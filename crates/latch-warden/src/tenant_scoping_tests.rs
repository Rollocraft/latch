use crate::supervisor_test_backends::*;
use crate::*;

#[test]
fn tenant_scoping_rejects_foreign_handles_and_isolates_global_kills() {
    let mut supervisor = Supervisor::with_external_authorization_and_audit();
    let acme = authorization("acme");
    let other = authorization("other");
    let calls = Calls::default();
    let (a, _) = register(&mut supervisor, &acme, "same-id", &calls);
    let (b, b_probe) = register(&mut supervisor, &other, "same-id", &calls);
    assert_ne!(a, b);
    supervisor.prepare(&other, &b).unwrap();
    supervisor.start(&other, &b).unwrap();
    calls.borrow_mut().clear();
    for (authorization, foreign) in [(&acme, &b), (&other, &a)] {
        assert!(matches!(
            supervisor.prepare(authorization, foreign),
            Err(SupervisorError::OrganizationMismatch)
        ));
        assert!(matches!(
            supervisor.start(authorization, foreign),
            Err(SupervisorError::OrganizationMismatch)
        ));
        assert!(matches!(
            supervisor.freeze(authorization, foreign),
            Err(SupervisorError::OrganizationMismatch)
        ));
        assert!(matches!(
            supervisor.terminate(authorization, foreign),
            Err(SupervisorError::OrganizationMismatch)
        ));
        assert!(matches!(
            supervisor.status(authorization, foreign),
            Err(SupervisorError::OrganizationMismatch)
        ));
    }
    assert!(calls.borrow().is_empty());
    let report = supervisor.kill_organization(&acme);
    assert_eq!(report.sessions.len(), 1);
    assert_eq!(report.sessions[0].handle, a);
    assert_eq!(
        supervisor.status(&other, &b).unwrap().state,
        SandboxState::Running
    );
    assert!(b_probe.allocated.get());
    assert!(!supervisor.is_stopped(&other));
    let (new, _) = register(&mut supervisor, &other, "new", &calls);
    supervisor.prepare(&other, &new).unwrap();
    supervisor.start(&other, &new).unwrap();
    supervisor.terminate(&other, &b).unwrap();
}

#[test]
fn duplicate_ids_never_replace_live_failed_or_terminated_sessions() {
    let mut supervisor = Supervisor::with_external_authorization_and_audit();
    let authorization = authorization("acme");
    let calls = Calls::default();
    let (handle, probe) = register(&mut supervisor, &authorization, "session", &calls);
    supervisor.prepare(&authorization, &handle).unwrap();
    supervisor.start(&authorization, &handle).unwrap();
    for state in [
        SandboxState::Running,
        SandboxState::Failed {
            operation: Operation::Terminate,
        },
        SandboxState::Terminated,
    ] {
        match state {
            SandboxState::Failed { .. } => {
                probe.failure.set(Some(Operation::Terminate));
                assert!(supervisor.terminate(&authorization, &handle).is_err());
            }
            SandboxState::Terminated => {
                probe.failure.set(None);
                supervisor.terminate(&authorization, &handle).unwrap();
            }
            _ => {}
        }
        calls.borrow_mut().clear();
        let (backend, _) = MockBackend::new("replacement", &calls);
        assert!(matches!(
            supervisor.register(&authorization, "session", backend, limits(), requirements()),
            Err(SupervisorError::DuplicateSession)
        ));
        assert!(calls.borrow().is_empty());
        assert_eq!(
            supervisor.status(&authorization, &handle).unwrap().state,
            state
        );
    }
}
