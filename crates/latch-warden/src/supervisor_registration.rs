use crate::{ExternalAuthorization, Sandbox, SessionHandle, SessionStatus, SupervisorError};
use latch_sandbox::SandboxBackend;
use latch_substrate::{EnforcementRequirements, ResourceLimits};
use std::collections::BTreeMap;

pub(crate) struct Organization<B: SandboxBackend> {
    pub(crate) stopped: bool,
    pub(crate) sessions: BTreeMap<String, crate::Sandbox<B>>,
}

impl<B: SandboxBackend> Default for Organization<B> {
    fn default() -> Self {
        Self {
            stopped: false,
            sessions: BTreeMap::new(),
        }
    }
}

pub struct Supervisor<B: SandboxBackend> {
    pub(crate) organizations: BTreeMap<String, Organization<B>>,
}

impl<B: SandboxBackend> Supervisor<B> {
    pub fn with_external_authorization_and_audit() -> Self {
        Self {
            organizations: BTreeMap::new(),
        }
    }

    pub fn is_stopped(&self, authorization: &ExternalAuthorization) -> bool {
        self.organizations
            .get(authorization.organization())
            .is_some_and(|organization| organization.stopped)
    }

    /// Registers a session backend under latched, organization-scoped supervision.
    pub fn register(
        &mut self,
        authorization: &ExternalAuthorization,
        session_id: impl Into<String>,
        backend: B,
        limits: ResourceLimits,
        requirements: EnforcementRequirements,
    ) -> Result<SessionHandle, SupervisorError<B::Error>> {
        self.require_active(authorization)?;
        let session_id = session_id.into();
        crate::validate_identifier(&session_id).map_err(SupervisorError::InvalidIdentifier)?;
        let organization = self
            .organizations
            .entry(authorization.organization().to_string())
            .or_default();
        if organization.sessions.contains_key(&session_id) {
            return Err(SupervisorError::DuplicateSession);
        }
        let mut sandbox = Sandbox::new(backend);
        sandbox.plan(limits, requirements)?;
        organization.sessions.insert(session_id.clone(), sandbox);
        Ok(SessionHandle {
            organization: authorization.organization().to_string(),
            session_id,
        })
    }
}

pub(crate) fn status_of<B: SandboxBackend>(sandbox: &Sandbox<B>) -> SessionStatus {
    SessionStatus {
        state: sandbox.state(),
        enforcement: sandbox.enforcement_status(),
    }
}
