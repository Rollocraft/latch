use crate::supervisor_test_backends::*;
use crate::*;

#[test]
fn organization_kill_is_sorted_complete_latched_and_retries_only_failures() {
    let mut supervisor = Supervisor::with_external_authorization_and_audit();
    let authorization = authorization("acme");
    let calls = Calls::default();
    let (z, z_probe) = register(&mut supervisor, &authorization, "z", &calls);
    let (a, a_probe) = register(&mut supervisor, &authorization, "a", &calls);
    let (m, _) = register(&mut supervisor, &authorization, "m", &calls);
    supervisor.prepare(&authorization, &a).unwrap();
    supervisor.start(&authorization, &a).unwrap();
    supervisor.prepare(&authorization, &z).unwrap();
    a_probe.failure.set(Some(Operation::Terminate));
    z_probe.failure.set(Some(Operation::Terminate));
    calls.borrow_mut().clear();
    let report = supervisor.kill_organization(&authorization);
    assert_eq!(report.organization, "acme");
    assert!(!report.all_terminated());
    assert!(supervisor.is_stopped(&authorization));
    assert_eq!(
        report
            .sessions
            .iter()
            .map(|s| s.handle.session_id())
            .collect::<Vec<_>>(),
        ["a", "m", "z"]
    );
    assert!(report.sessions[0].result.is_err());
    assert!(report.sessions[1].result.is_ok());
    assert!(report.sessions[2].result.is_err());
    assert_eq!(
        report.sessions[0].status.enforcement,
        EnforcementStatus::Unknown
    );
    assert_eq!(
        *calls.borrow(),
        [
            ("a".into(), Operation::Terminate),
            ("m".into(), Operation::Terminate),
            ("z".into(), Operation::Terminate)
        ]
    );
    for handle in [&a, &m, &z] {
        assert!(matches!(
            supervisor.start(&authorization, handle),
            Err(SupervisorError::OrganizationStopped)
        ));
        assert!(matches!(
            supervisor.prepare(&authorization, handle),
            Err(SupervisorError::OrganizationStopped)
        ));
    }
    let (backend, _) = MockBackend::new("new", &calls);
    assert!(matches!(
        supervisor.register(&authorization, "new", backend, limits(), requirements()),
        Err(SupervisorError::OrganizationStopped)
    ));
    calls.borrow_mut().clear();
    a_probe.failure.set(None);
    let report = supervisor.kill_organization(&authorization);
    assert!(!report.all_terminated());
    assert_eq!(
        *calls.borrow(),
        [
            ("a".into(), Operation::Terminate),
            ("z".into(), Operation::Terminate)
        ]
    );
    calls.borrow_mut().clear();
    z_probe.failure.set(None);
    assert!(
        supervisor
            .kill_organization(&authorization)
            .all_terminated()
    );
    assert_eq!(*calls.borrow(), [("z".into(), Operation::Terminate)]);
    calls.borrow_mut().clear();
    assert!(
        supervisor
            .kill_organization(&authorization)
            .all_terminated()
    );
    assert!(calls.borrow().is_empty());
    assert!(!a_probe.allocated.get());
    assert!(!z_probe.allocated.get());
    assert!(supervisor.is_stopped(&authorization));
}

#[test]
fn empty_organization_kill_prevents_first_registration_without_backend_calls() {
    let mut supervisor = Supervisor::with_external_authorization_and_audit();
    let authorization = authorization("acme");
    assert!(!supervisor.is_stopped(&authorization));
    let report = supervisor.kill_organization(&authorization);
    assert!(report.sessions.is_empty());
    assert!(report.all_terminated());
    let calls = Calls::default();
    let (backend, _) = MockBackend::new("session", &calls);
    assert!(matches!(
        supervisor.register(&authorization, "session", backend, limits(), requirements()),
        Err(SupervisorError::OrganizationStopped)
    ));
    assert!(calls.borrow().is_empty());
}
