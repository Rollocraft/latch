use crate::supervisor_test_backends::*;
use crate::*;

#[test]
fn registration_validation_is_fail_closed_and_failed_planning_can_be_retried() {
    for invalid in ["", " ", "a\nb", "a b", &"a".repeat(257)] {
        assert!(ExternalAuthorization::from_trusted_control_plane(invalid).is_err());
        let mut supervisor = Supervisor::with_external_authorization_and_audit();
        let calls = Calls::default();
        let (backend, _) = MockBackend::new("invalid", &calls);
        assert!(matches!(
            supervisor.register(
                &authorization("acme"),
                invalid,
                backend,
                limits(),
                requirements()
            ),
            Err(SupervisorError::InvalidIdentifier(_))
        ));
        assert!(calls.borrow().is_empty());
    }
    let mut unsupported = Supervisor::with_external_authorization_and_audit();
    assert!(matches!(
        unsupported.register(
            &authorization("acme"),
            "session",
            UnsupportedBackend,
            limits(),
            EnforcementRequirements::protected()
        ),
        Err(SupervisorError::Sandbox(SandboxError::Unsupported(_)))
    ));
    let mut supervisor = Supervisor::with_external_authorization_and_audit();
    let calls = Calls::default();
    let (backend, probe) = MockBackend::new("session", &calls);
    probe.failure.set(Some(Operation::Plan));
    assert!(matches!(
        supervisor.register(
            &authorization("acme"),
            "session",
            backend,
            limits(),
            requirements()
        ),
        Err(SupervisorError::Sandbox(SandboxError::Backend {
            operation: Operation::Plan,
            ..
        }))
    ));
    assert!(!probe.allocated.get());
    register(&mut supervisor, &authorization("acme"), "session", &calls);
}
