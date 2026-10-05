use crate::{SessionHandle, SessionStatus};
use latch_sandbox::SandboxError;

/// Outcome of terminating one session during a kill.
#[derive(Debug)]
pub struct SessionTermination<E> {
    pub handle: SessionHandle,
    pub status: SessionStatus,
    pub result: Result<(), SandboxError<E>>,
}

/// Per-session results of latching and killing a whole organization.
#[derive(Debug)]
pub struct OrganizationKillReport<E> {
    pub organization: String,
    pub sessions: Vec<SessionTermination<E>>,
}

impl<E> OrganizationKillReport<E> {
    pub fn all_terminated(&self) -> bool {
        self.sessions.iter().all(|session| {
            session.result.is_ok()
                && session.status.state == latch_sandbox::SandboxState::Terminated
        })
    }
}
