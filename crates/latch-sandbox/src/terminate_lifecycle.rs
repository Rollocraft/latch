use crate::{Operation, SandboxBackend, SandboxError, SandboxManager, SandboxState};

impl<B: SandboxBackend> SandboxManager<B> {
    pub fn terminate(&mut self) -> Result<(), SandboxError<B::Error>> {
        if matches!(
            self.state,
            SandboxState::Unplanned | SandboxState::Terminated
        ) {
            return Err(SandboxError::InvalidTransition {
                state: self.state,
                operation: Operation::Terminate,
            });
        }
        let result = self.backend.terminate_all_and_cleanup();
        self.complete(Operation::Terminate, SandboxState::Terminated, result)
    }
}
