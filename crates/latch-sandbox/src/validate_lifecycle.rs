use crate::{Operation, SandboxBackend, SandboxError, SandboxManager, SandboxPlan, SandboxState};

impl<B: SandboxBackend> SandboxManager<B> {
    pub(crate) fn require_state(
        &self,
        operation: Operation,
        allowed: &[SandboxState],
    ) -> Result<(), SandboxError<B::Error>> {
        if allowed.contains(&self.state) {
            Ok(())
        } else {
            Err(SandboxError::InvalidTransition {
                state: self.state,
                operation,
            })
        }
    }

    pub(crate) fn validate(
        &self,
        plan: &SandboxPlan,
        operation: Operation,
    ) -> Result<(), SandboxError<B::Error>> {
        plan.requirements
            .validate(&self.backend.capabilities())
            .map_err(SandboxError::Unsupported)?;
        self.backend
            .validate_plan(plan)
            .map_err(|source| SandboxError::Backend { operation, source })
    }

    pub(crate) fn validated_plan(
        &mut self,
        operation: Operation,
    ) -> Result<SandboxPlan, SandboxError<B::Error>> {
        let Some(plan) = self.plan else {
            return Err(SandboxError::InvalidTransition {
                state: self.state,
                operation,
            });
        };
        if let Err(error) = self.validate(&plan, operation) {
            self.fail(operation);
            return Err(error);
        }
        Ok(plan)
    }
}
