use latch_sandbox::{EnforcementStatus, SandboxState};

/// Opaque pairing of an organization with one registered session id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionHandle {
    pub(crate) organization: String,
    pub(crate) session_id: String,
}

impl SessionHandle {
    pub fn organization(&self) -> &str {
        &self.organization
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

/// Sandbox state paired with the enforcement level actually reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionStatus {
    pub state: SandboxState,
    pub enforcement: EnforcementStatus,
}
