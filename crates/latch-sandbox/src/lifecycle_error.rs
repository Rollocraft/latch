use crate::{Operation, SandboxState, UnsupportedRequirements};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SandboxError<E> {
    InvalidTransition {
        state: SandboxState,
        operation: Operation,
    },
    Unsupported(UnsupportedRequirements),
    Backend {
        operation: Operation,
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for SandboxError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { state, operation } => {
                write!(f, "cannot {operation:?} sandbox in state {state:?}")
            }
            Self::Unsupported(error) => error.fmt(f),
            Self::Backend { operation, source } => {
                write!(f, "backend failed during {operation:?}: {source}")
            }
        }
    }
}

impl<E: Error + 'static> Error for SandboxError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Unsupported(error) => Some(error),
            Self::Backend { source, .. } => Some(source),
            Self::InvalidTransition { .. } => None,
        }
    }
}
