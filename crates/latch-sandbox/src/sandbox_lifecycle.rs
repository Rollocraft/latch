use crate::{EnforcementStatus, SandboxBackend, SandboxPlan, SandboxState};

pub struct SandboxLifecycle<B: SandboxBackend> {
    pub(crate) backend: B,
    pub(crate) state: SandboxState,
    pub(crate) plan: Option<SandboxPlan>,
    pub(crate) enforcement: EnforcementStatus,
}

impl<B: SandboxBackend> SandboxLifecycle<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            state: SandboxState::Unplanned,
            plan: None,
            enforcement: EnforcementStatus::NotEstablished,
        }
    }

    pub fn state(&self) -> SandboxState {
        self.state
    }

    pub fn enforcement_status(&self) -> EnforcementStatus {
        self.enforcement
    }

    pub fn current_plan(&self) -> Option<&SandboxPlan> {
        self.plan.as_ref()
    }
}
