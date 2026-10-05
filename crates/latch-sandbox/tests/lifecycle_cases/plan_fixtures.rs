use super::*;
use std::time::Duration;

pub(super) fn limits() -> ResourceLimits {
    ResourceLimits::new(
        Duration::from_secs(2),
        1024,
        2,
        4096,
        Duration::from_secs(5),
    )
    .unwrap()
}

pub(super) fn planned(mock: &MockBackend) -> SandboxManager<MockBackend> {
    let mut manager = SandboxManager::new(mock.clone());
    manager
        .plan(limits(), EnforcementRequirements::protected())
        .unwrap();
    manager
}
