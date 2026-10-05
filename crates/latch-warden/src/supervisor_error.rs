use crate::external_authorization::InvalidIdentifier;
use latch_sandbox::SandboxError;
use std::error::Error;
use std::fmt;

/// Every way a supervision request can be refused or fail.
#[derive(Debug)]
pub enum SupervisorError<E> {
    InvalidIdentifier(InvalidIdentifier),
    OrganizationStopped,
    OrganizationMismatch,
    DuplicateSession,
    SessionNotFound,
    Sandbox(SandboxError<E>),
}

impl<E: fmt::Display> fmt::Display for SupervisorError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(error) => error.fmt(f),
            Self::OrganizationStopped => f.write_str("organization stop is latched"),
            Self::OrganizationMismatch => f.write_str("session belongs to another organization"),
            Self::DuplicateSession => f.write_str("session identifier is already registered"),
            Self::SessionNotFound => f.write_str("session is not registered"),
            Self::Sandbox(error) => error.fmt(f),
        }
    }
}

impl<E: Error + 'static> Error for SupervisorError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidIdentifier(error) => Some(error),
            Self::Sandbox(error) => Some(error),
            _ => None,
        }
    }
}

impl<E> From<SandboxError<E>> for SupervisorError<E> {
    fn from(error: SandboxError<E>) -> Self {
        Self::Sandbox(error)
    }
}
