use crate::{
    ExternalAuthorization, OrganizationKillReport, SessionHandle, SessionStatus,
    SessionTermination, Supervisor, supervisor_lifecycle::terminate_sandbox,
    supervisor_registration::status_of,
};
use latch_sandbox::SandboxBackend;

impl<B: SandboxBackend> Supervisor<B> {
    /// Read-only status lookup; requires matching organization scoping.
    pub fn status(
        &self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<SessionStatus, crate::SupervisorError<B::Error>> {
        self.require_organization(authorization, handle)?;
        let sandbox = self
            .organizations
            .get(authorization.organization())
            .and_then(|organization| organization.sessions.get(handle.session_id()))
            .ok_or(crate::SupervisorError::SessionNotFound)?;
        Ok(status_of(sandbox))
    }

    /// Latches the organization stopped and terminates every live session.
    pub fn kill_organization(
        &mut self,
        authorization: &ExternalAuthorization,
    ) -> OrganizationKillReport<B::Error> {
        let organization = self
            .organizations
            .entry(authorization.organization().to_string())
            .or_default();
        organization.stopped = true;
        let sessions = organization
            .sessions
            .iter_mut()
            .map(|(session_id, sandbox)| {
                let result = terminate_sandbox(sandbox);
                SessionTermination {
                    handle: SessionHandle {
                        organization: authorization.organization().to_string(),
                        session_id: session_id.clone(),
                    },
                    status: status_of(sandbox),
                    result,
                }
            })
            .collect();
        OrganizationKillReport {
            organization: authorization.organization().to_string(),
            sessions,
        }
    }
}
