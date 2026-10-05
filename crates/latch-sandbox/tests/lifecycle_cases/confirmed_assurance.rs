use super::*;

#[test]
fn lifecycle_reports_only_backend_confirmed_assurance() {
    for assurance in [
        AssuranceLevel::CooperativeMediation,
        AssuranceLevel::OsConfinement,
    ] {
        let mock = MockBackend::new(assurance);
        let mut manager = SandboxManager::new(mock.clone());
        assert_eq!(manager.state(), SandboxState::Unplanned);
        assert!(manager.current_plan().is_none());
        manager
            .plan(limits(), EnforcementRequirements::new(assurance))
            .unwrap();
        assert_eq!(manager.state(), SandboxState::Planned);
        assert_eq!(manager.current_plan().unwrap().limits(), limits());
        assert_eq!(
            manager.current_plan().unwrap().requirements().assurance(),
            assurance
        );
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::NotEstablished
        );
        manager.prepare().unwrap();
        assert_eq!(manager.state(), SandboxState::Prepared);
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::NotEstablished
        );
        manager.start().unwrap();
        assert_eq!(manager.state(), SandboxState::Running);
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::BackendReported { assurance }
        );
        manager.freeze().unwrap();
        assert_eq!(manager.state(), SandboxState::Frozen);
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::BackendReported { assurance }
        );
        manager.terminate().unwrap();
        assert_eq!(manager.state(), SandboxState::Terminated);
        assert_eq!(
            manager.enforcement_status(),
            EnforcementStatus::NotEstablished
        );
        assert_eq!(
            *mock.calls.borrow(),
            [
                Operation::Prepare,
                Operation::Start,
                Operation::Freeze,
                Operation::Terminate
            ]
        );
    }
}
