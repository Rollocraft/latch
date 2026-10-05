use super::*;

#[test]
fn failed_operations_never_report_success_and_allow_cleanup() {
    for operation in [
        Operation::Prepare,
        Operation::Start,
        Operation::Freeze,
        Operation::Terminate,
    ] {
        let mock = MockBackend::new(AssuranceLevel::OsConfinement);
        let mut manager = planned(&mock);
        if operation != Operation::Prepare {
            manager.prepare().unwrap();
        }
        if matches!(operation, Operation::Freeze | Operation::Terminate) {
            manager.start().unwrap();
        }
        mock.failure.set(Some(operation));
        let result = match operation {
            Operation::Prepare => manager.prepare(),
            Operation::Start => manager.start(),
            Operation::Freeze => manager.freeze(),
            Operation::Terminate => manager.terminate(),
            Operation::Plan => unreachable!(),
        };
        assert!(
            matches!(result, Err(SandboxError::Backend { operation: failed, source: MockError }) if failed == operation)
        );
        assert_eq!(manager.state(), SandboxState::Failed { operation });
        assert_eq!(manager.enforcement_status(), EnforcementStatus::Unknown);
        let calls = mock.calls.borrow().len();
        assert!(manager.prepare().is_err());
        assert!(manager.start().is_err());
        assert!(manager.freeze().is_err());
        assert!(
            manager
                .plan(limits(), EnforcementRequirements::protected())
                .is_err()
        );
        assert_eq!(mock.calls.borrow().len(), calls);
        mock.failure.set(Some(Operation::Terminate));
        assert!(manager.terminate().is_err());
        assert_eq!(
            manager.state(),
            SandboxState::Failed {
                operation: Operation::Terminate
            }
        );
        assert_eq!(manager.enforcement_status(), EnforcementStatus::Unknown);
        mock.failure.set(None);
        manager.terminate().unwrap();
        assert_eq!(manager.state(), SandboxState::Terminated);
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::NotEstablished
        );
    }
}
