use crate::{
    EnforcementStatus, Operation, SandboxBackend, SandboxError, SandboxManager, SandboxState,
};

impl<B: SandboxBackend> SandboxManager<B> {
    pub(crate) fn fail(&mut self, operation: Operation) {
        self.state = SandboxState::Failed { operation };
        self.enforcement = EnforcementStatus::Unknown;
    }

    pub(crate) fn complete(
        &mut self,
        operation: Operation,
        next: SandboxState,
        result: Result<(), B::Error>,
    ) -> Result<(), SandboxError<B::Error>> {
        match result {
            Ok(()) => {
                self.state = next;
                self.enforcement = match (next, self.plan) {
                    (SandboxState::Running | SandboxState::Frozen, Some(plan)) => {
                        EnforcementStatus::BackendReported {
                            assurance: plan.requirements.assurance(),
                        }
                    }
                    _ => EnforcementStatus::NotEstablished,
                };
                Ok(())
            }
            Err(source) => {
                self.fail(operation);
                Err(SandboxError::Backend { operation, source })
            }
        }
    }
}
