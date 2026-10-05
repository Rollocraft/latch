use crate::{
    EnforcementRequirements, Operation, ResourceLimits, SandboxBackend, SandboxError,
    SandboxManager, SandboxPlan, SandboxState,
};

impl<B: SandboxBackend> SandboxManager<B> {
    pub fn plan(
        &mut self,
        limits: ResourceLimits,
        requirements: EnforcementRequirements,
    ) -> Result<&SandboxPlan, SandboxError<B::Error>> {
        self.require_state(Operation::Plan, &[SandboxState::Unplanned])?;
        let plan = SandboxPlan {
            limits,
            requirements,
        };
        self.validate(&plan, Operation::Plan)?;
        self.state = SandboxState::Planned;
        Ok(self.plan.insert(plan))
    }

    pub fn prepare(&mut self) -> Result<(), SandboxError<B::Error>> {
        self.require_state(Operation::Prepare, &[SandboxState::Planned])?;
        let plan = self.validated_plan(Operation::Prepare)?;
        let result = self.backend.prepare_enforcement(&plan);
        self.complete(Operation::Prepare, SandboxState::Prepared, result)
    }

    pub fn start(&mut self) -> Result<(), SandboxError<B::Error>> {
        self.require_state(Operation::Start, &[SandboxState::Prepared])?;
        self.validated_plan(Operation::Start)?;
        let result = self.backend.start_enforced();
        self.complete(Operation::Start, SandboxState::Running, result)
    }

    pub fn freeze(&mut self) -> Result<(), SandboxError<B::Error>> {
        self.require_state(Operation::Freeze, &[SandboxState::Running])?;
        self.validated_plan(Operation::Freeze)?;
        let result = self.backend.freeze_all();
        self.complete(Operation::Freeze, SandboxState::Frozen, result)
    }
}
