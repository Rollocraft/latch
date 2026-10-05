use super::*;

#[test]
fn cooperative_backend_cannot_plan_protected_agents() {
    let mock = MockBackend::new(AssuranceLevel::CooperativeMediation);
    let mut manager = SandboxManager::new(mock.clone());
    assert!(matches!(
        manager.plan(limits(), EnforcementRequirements::protected()),
        Err(SandboxError::Unsupported(_))
    ));
    assert!(manager.start().is_err());
    assert!(mock.calls.borrow().is_empty());
    assert_eq!(manager.state(), SandboxState::Unplanned);
    assert_eq!(
        manager.enforcement_status(),
        EnforcementStatus::NotEstablished
    );
}

#[test]
fn planner_rejects_each_missing_control_without_backend_effects() {
    for missing in Control::ALL {
        let mock = MockBackend::new(AssuranceLevel::OsConfinement);
        mock.caps.set(
            Control::ALL
                .into_iter()
                .filter(|&c| c != missing)
                .fold(BackendCapabilities::default(), |caps, control| {
                    caps.with_control(control, AssuranceLevel::OsConfinement)
                }),
        );
        let mut manager = SandboxManager::new(mock.clone());
        let Err(SandboxError::Unsupported(error)) =
            manager.plan(limits(), EnforcementRequirements::protected())
        else {
            panic!("missing control accepted");
        };
        assert_eq!(error.gaps().len(), 1);
        assert_eq!(error.gaps()[0].control, missing);
        assert!(manager.prepare().is_err());
        assert!(manager.start().is_err());
        assert!(mock.calls.borrow().is_empty());
    }
}
