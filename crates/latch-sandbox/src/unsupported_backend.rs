use crate::{BackendCapabilities, SandboxBackend, SandboxPlan};
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendUnavailable;

impl fmt::Display for BackendUnavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("no OS confinement backend is implemented")
    }
}

impl Error for BackendUnavailable {}

#[derive(Debug, Default)]
pub struct UnsupportedBackend;

impl SandboxBackend for UnsupportedBackend {
    type Error = BackendUnavailable;

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::default()
    }

    fn validate_plan(&self, _plan: &SandboxPlan) -> Result<(), Self::Error> {
        Err(BackendUnavailable)
    }

    fn prepare_enforcement(&mut self, _plan: &SandboxPlan) -> Result<(), Self::Error> {
        Err(BackendUnavailable)
    }

    fn start_enforced(&mut self) -> Result<(), Self::Error> {
        Err(BackendUnavailable)
    }

    fn freeze_all(&mut self) -> Result<(), Self::Error> {
        Err(BackendUnavailable)
    }

    fn terminate_all_and_cleanup(&mut self) -> Result<(), Self::Error> {
        Err(BackendUnavailable)
    }
}
