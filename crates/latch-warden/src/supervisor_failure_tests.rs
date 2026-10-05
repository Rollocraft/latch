use crate::supervisor_test_backends::*;
use crate::*;

#[test]
fn every_lifecycle_failure_preserves_unknown_state_until_cleanup_succeeds() {
    for operation in [
        Operation::Prepare,
        Operation::Start,
        Operation::Freeze,
        Operation::Terminate,
    ] {
        let mut supervisor = Supervisor::with_external_authorization_and_audit();
        let authorization = authorization("acme");
        let calls = Calls::default();
        let (handle, probe) = register(&mut supervisor, &authorization, "session", &calls);
        if operation != Operation::Prepare {
            supervisor.prepare(&authorization, &handle).unwrap();
        }
        if matches!(operation, Operation::Freeze | Operation::Terminate) {
            supervisor.start(&authorization, &handle).unwrap();
        }
        probe.failure.set(Some(operation));
        let result = match operation {
            Operation::Prepare => supervisor.prepare(&authorization, &handle),
            Operation::Start => supervisor.start(&authorization, &handle),
            Operation::Freeze => supervisor.freeze(&authorization, &handle),
            Operation::Terminate => supervisor.terminate(&authorization, &handle),
            Operation::Plan => unreachable!(),
        };
        assert!(
            matches!(result, Err(SupervisorError::Sandbox(SandboxError::Backend {
            operation: failed, source: MockError,
        })) if failed == operation)
        );
        assert_eq!(
            supervisor.status(&authorization, &handle).unwrap(),
            SessionStatus {
                state: SandboxState::Failed { operation },
                enforcement: EnforcementStatus::Unknown,
            }
        );
        assert!(probe.allocated.get());
        probe.failure.set(Some(Operation::Terminate));
        assert!(supervisor.terminate(&authorization, &handle).is_err());
        assert!(supervisor.terminate(&authorization, &handle).is_err());
        assert!(probe.allocated.get());
        probe.failure.set(None);
        assert_eq!(
            supervisor.terminate(&authorization, &handle).unwrap().state,
            SandboxState::Terminated
        );
        assert!(!probe.allocated.get());
    }
}
