//! Organization-scoped sandbox supervision over the latch-sandbox backend.
//!
//! The supervisor keeps per-organization latches (stopped state), prevents
//! cross-tenant handle use, and reports every termination individually.

mod external_authorization;
mod session_handle;
mod supervisor_error;
mod supervisor_kill;
mod supervisor_lifecycle;
mod supervisor_registration;
mod supervisor_session_access;
mod termination_reports;

pub use external_authorization::{ExternalAuthorization, InvalidIdentifier};
pub use latch_sandbox::{EnforcementStatus, SandboxState};
pub use session_handle::{SessionHandle, SessionStatus};
pub use supervisor_error::SupervisorError;
pub use supervisor_registration::Supervisor;
pub use termination_reports::{OrganizationKillReport, SessionTermination};

pub(crate) use external_authorization::validate_identifier;
pub(crate) use latch_sandbox::SandboxError;
pub(crate) use latch_sandbox::SandboxManager as Sandbox;
#[cfg(test)]
pub(crate) use latch_substrate::{EnforcementRequirements, ResourceLimits};

#[cfg(test)]
mod supervisor_test_backends;
#[cfg(test)]
mod supervisor_test_fixtures;
#[cfg(test)]
mod tests;
