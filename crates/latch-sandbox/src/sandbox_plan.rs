use crate::{BackendCapabilities, EnforcementRequirements, ResourceLimits};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxPlan {
    pub(crate) limits: ResourceLimits,
    pub(crate) requirements: EnforcementRequirements,
}

impl SandboxPlan {
    pub fn limits(&self) -> ResourceLimits {
        self.limits
    }

    pub fn requirements(&self) -> EnforcementRequirements {
        self.requirements
    }
}

pub trait SandboxBackend {
    type Error: Error + Send + Sync + 'static;

    fn capabilities(&self) -> BackendCapabilities;
    fn validate_plan(&self, plan: &SandboxPlan) -> Result<(), Self::Error>;
    fn prepare_enforcement(&mut self, plan: &SandboxPlan) -> Result<(), Self::Error>;
    fn start_enforced(&mut self) -> Result<(), Self::Error>;
    fn freeze_all(&mut self) -> Result<(), Self::Error>;
    fn terminate_all_and_cleanup(&mut self) -> Result<(), Self::Error>;
}
