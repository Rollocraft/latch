use super::*;

#[test]
fn invalid_transitions_do_not_call_backend_or_change_status() {
    let mock = MockBackend::new(AssuranceLevel::OsConfinement);
    let mut manager = SandboxManager::new(mock.clone());
    let states = [
        SandboxState::Unplanned,
        SandboxState::Planned,
        SandboxState::Prepared,
        SandboxState::Running,
        SandboxState::Frozen,
        SandboxState::Terminated,
    ];
    for state in states {
        match state {
            SandboxState::Unplanned => {}
            SandboxState::Planned => {
                manager
                    .plan(limits(), EnforcementRequirements::protected())
                    .unwrap();
            }
            SandboxState::Prepared => manager.prepare().unwrap(),
            SandboxState::Running => manager.start().unwrap(),
            SandboxState::Frozen => manager.freeze().unwrap(),
            SandboxState::Terminated => manager.terminate().unwrap(),
            SandboxState::Failed { .. } => unreachable!(),
        }
        let status = manager.enforcement_status();
        let calls = mock.calls.borrow().len();
        for operation in [
            Operation::Plan,
            Operation::Prepare,
            Operation::Start,
            Operation::Freeze,
            Operation::Terminate,
        ] {
            let allowed = matches!(
                (state, operation),
                (SandboxState::Unplanned, Operation::Plan)
                    | (
                        SandboxState::Planned,
                        Operation::Prepare | Operation::Terminate
                    )
                    | (
                        SandboxState::Prepared,
                        Operation::Start | Operation::Terminate
                    )
                    | (
                        SandboxState::Running,
                        Operation::Freeze | Operation::Terminate
                    )
                    | (SandboxState::Frozen, Operation::Terminate)
            );
            if allowed {
                continue;
            }
            let result = match operation {
                Operation::Plan => manager
                    .plan(limits(), EnforcementRequirements::protected())
                    .map(|_| ()),
                Operation::Prepare => manager.prepare(),
                Operation::Start => manager.start(),
                Operation::Freeze => manager.freeze(),
                Operation::Terminate => manager.terminate(),
            };
            assert!(
                matches!(result, Err(SandboxError::InvalidTransition { state: actual, operation: attempted }) if actual == state && attempted == operation)
            );
            assert_eq!(manager.state(), state);
            assert_eq!(manager.enforcement_status(), status);
            assert_eq!(mock.calls.borrow().len(), calls);
        }
    }
}
