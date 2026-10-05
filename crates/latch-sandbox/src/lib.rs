mod complete_lifecycle;
mod lifecycle_error;
mod lifecycle_state;
mod plan_lifecycle;
mod sandbox_lifecycle;
mod sandbox_plan;
mod terminate_lifecycle;
mod unsupported_backend;
mod validate_lifecycle;

pub use latch_substrate::{
    AssuranceLevel, BackendCapabilities, Control, EnforcementRequirements, ResourceLimits,
    UnsupportedRequirements,
};
pub use lifecycle_error::SandboxError;
pub use lifecycle_state::{EnforcementStatus, Operation, SandboxState};
/// Drives one sandbox through its lifecycle on a [`SandboxBackend`].
pub use sandbox_lifecycle::SandboxLifecycle as SandboxManager;
pub use sandbox_plan::{SandboxBackend, SandboxPlan};
pub use unsupported_backend::{BackendUnavailable, UnsupportedBackend};

#[cfg(test)]
#[path = "../tests/lifecycle_cases/test_suite.rs"]
mod tests;
