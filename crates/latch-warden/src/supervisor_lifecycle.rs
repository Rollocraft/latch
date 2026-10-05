use crate::{
    ExternalAuthorization, Sandbox, SessionHandle, SessionStatus, Supervisor,
    supervisor_registration::status_of,
};
use latch_sandbox::{SandboxBackend, SandboxState};

impl<B: SandboxBackend> Supervisor<B> {
    /// Prepares enforcement; refused once the organization is stopped.
    pub fn prepare(
        &mut self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<SessionStatus, crate::SupervisorError<B::Error>> {
        self.require_active(authorization)?;
        let sandbox = self.session_mut(authorization, handle)?;
        sandbox.prepare()?;
        Ok(status_of(sandbox))
    }

    /// Starts a prepared session; refused once the organization is stopped.
    pub fn start(
        &mut self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<SessionStatus, crate::SupervisorError<B::Error>> {
        self.require_active(authorization)?;
        let sandbox = self.session_mut(authorization, handle)?;
        sandbox.start()?;
        Ok(status_of(sandbox))
    }

    /// Freezes a running session; freezing a frozen session is idempotent.
    pub fn freeze(
        &mut self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<SessionStatus, crate::SupervisorError<B::Error>> {
        let sandbox = self.session_mut(authorization, handle)?;
        if sandbox.state() != SandboxState::Frozen {
            sandbox.freeze()?;
        }
        Ok(status_of(sandbox))
    }

    /// Terminates a session; terminating a terminated session is idempotent.
    pub fn terminate(
        &mut self,
        authorization: &ExternalAuthorization,
        handle: &SessionHandle,
    ) -> Result<SessionStatus, crate::SupervisorError<B::Error>> {
        let sandbox = self.session_mut(authorization, handle)?;
        terminate_sandbox(sandbox)?;
        Ok(status_of(sandbox))
    }
}

pub(crate) fn terminate_sandbox<B: SandboxBackend>(
    sandbox: &mut Sandbox<B>,
) -> Result<(), crate::SandboxError<B::Error>> {
    if sandbox.state() == SandboxState::Terminated {
        Ok(())
    } else {
        sandbox.terminate()
    }
}
