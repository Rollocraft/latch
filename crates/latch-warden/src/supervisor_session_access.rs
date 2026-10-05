use crate::{ExternalAuthorization, Sandbox, SessionHandle, Supervisor, SupervisorError};
use latch_sandbox::SandboxBackend;

impl<B: SandboxBackend> Supervisor<B> {
    pub(crate) fn require_active(
        &self,
        authorization: &ExternalAuthorization,
    ) -> Result<(), SupervisorError<B::Error>> {
        if self.is_stopped(authorization) {
            Err(SupervisorError::OrganizationStopped)
        } else {
            Ok(())
        }
    }

    pub(crate) fn require_organization(
        &self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<(), SupervisorError<B::Error>> {
        if authorization.organization() != handle.organization() {
            Err(SupervisorError::OrganizationMismatch)
        } else {
            Ok(())
        }
    }

    pub(crate) fn session_mut(
        &mut self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<&mut Sandbox<B>, SupervisorError<B::Error>> {
        self.require_organization(authorization, handle)?;
        self.organizations
            .get_mut(authorization.organization())
            .and_then(|organization| organization.sessions.get_mut(handle.session_id()))
            .ok_or(SupervisorError::SessionNotFound)
    }
}
