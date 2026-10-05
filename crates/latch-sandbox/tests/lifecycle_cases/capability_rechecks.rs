use super::*;

#[test]
fn capabilities_and_specific_plan_are_rechecked_before_effects() {
    for operation in [Operation::Prepare, Operation::Start, Operation::Freeze] {
        for lose_capabilities in [true, false] {
            let mock = MockBackend::new(AssuranceLevel::OsConfinement);
            let mut manager = planned(&mock);
            if operation != Operation::Prepare {
                manager.prepare().unwrap();
            }
            if operation == Operation::Freeze {
                manager.start().unwrap();
            }
            let calls = mock.calls.borrow().len();
            if lose_capabilities {
                mock.caps.set(BackendCapabilities::default());
            } else {
                mock.reject_limits.set(true);
            }
            let result = match operation {
                Operation::Prepare => manager.prepare(),
                Operation::Start => manager.start(),
                Operation::Freeze => manager.freeze(),
                _ => unreachable!(),
            };
            if lose_capabilities {
                assert!(matches!(result, Err(SandboxError::Unsupported(_))));
            } else {
                assert!(matches!(result, Err(SandboxError::Backend { .. })));
            }
            assert_eq!(manager.state(), SandboxState::Failed { operation });
            assert_eq!(manager.enforcement_status(), EnforcementStatus::Unknown);
            assert_eq!(mock.calls.borrow().len(), calls);
            manager.terminate().unwrap();
            assert_eq!(manager.state(), SandboxState::Terminated);
        }
    }
}
