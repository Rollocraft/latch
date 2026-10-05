use super::*;

#[test]
fn termination_is_available_from_every_planned_nonterminal_state() {
    for steps in 0..=3 {
        let mock = MockBackend::new(AssuranceLevel::OsConfinement);
        let mut manager = planned(&mock);
        if steps >= 1 {
            manager.prepare().unwrap();
        }
        if steps >= 2 {
            manager.start().unwrap();
        }
        if steps >= 3 {
            manager.freeze().unwrap();
        }
        manager.terminate().unwrap();
        assert_eq!(manager.state(), SandboxState::Terminated);
        assert_eq!(mock.calls.borrow().last(), Some(&Operation::Terminate));
    }
}
