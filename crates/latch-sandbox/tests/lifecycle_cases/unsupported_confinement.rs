use super::*;

#[test]
fn unsupported_backend_never_launches() {
    for assurance in [
        AssuranceLevel::CooperativeMediation,
        AssuranceLevel::OsConfinement,
    ] {
        let mut manager = SandboxManager::new(UnsupportedBackend);
        assert!(matches!(
            manager.plan(limits(), EnforcementRequirements::new(assurance)),
            Err(SandboxError::Unsupported(_))
        ));
        assert_eq!(manager.state(), SandboxState::Unplanned);
        assert!(manager.current_plan().is_none());
        assert!(manager.prepare().is_err());
        assert!(manager.start().is_err());
        assert!(manager.freeze().is_err());
        assert!(manager.terminate().is_err());
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::NotEstablished
        );
    }
    let mut backend = UnsupportedBackend;
    let plan = SandboxPlan {
        limits: limits(),
        requirements: EnforcementRequirements::protected(),
    };
    assert_eq!(backend.validate_plan(&plan), Err(BackendUnavailable));
    assert_eq!(backend.prepare_enforcement(&plan), Err(BackendUnavailable));
    assert_eq!(backend.start_enforced(), Err(BackendUnavailable));
    assert_eq!(backend.freeze_all(), Err(BackendUnavailable));
    assert_eq!(backend.terminate_all_and_cleanup(), Err(BackendUnavailable));
}
