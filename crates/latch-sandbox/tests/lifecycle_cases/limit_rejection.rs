use super::*;

#[test]
fn backend_can_reject_specific_limits_before_preparation() {
    let mock = MockBackend::new(AssuranceLevel::OsConfinement);
    mock.reject_limits.set(true);
    let mut manager = SandboxManager::new(mock.clone());
    assert!(matches!(
        manager.plan(limits(), EnforcementRequirements::protected()),
        Err(SandboxError::Backend {
            operation: Operation::Plan,
            source: MockError
        })
    ));
    assert_eq!(manager.state(), SandboxState::Unplanned);
    assert!(manager.current_plan().is_none());
    assert_eq!(
        manager.enforcement_status(),
        EnforcementStatus::NotEstablished
    );
    assert!(mock.calls.borrow().is_empty());
    mock.reject_limits.set(false);
    assert!(
        manager
            .plan(limits(), EnforcementRequirements::protected())
            .is_ok()
    );
}
