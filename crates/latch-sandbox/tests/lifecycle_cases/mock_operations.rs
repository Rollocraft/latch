use super::*;

impl SandboxBackend for MockBackend {
    type Error = MockError;

    fn capabilities(&self) -> BackendCapabilities {
        self.caps.get()
    }

    fn validate_plan(&self, plan: &SandboxPlan) -> Result<(), Self::Error> {
        assert_eq!(plan.limits(), limits());
        if self.reject_limits.get() {
            Err(MockError)
        } else {
            Ok(())
        }
    }

    fn prepare_enforcement(&mut self, plan: &SandboxPlan) -> Result<(), Self::Error> {
        self.prepared = Some(*plan);
        self.perform(Operation::Prepare)
    }

    fn start_enforced(&mut self) -> Result<(), Self::Error> {
        assert!(self.prepared.is_some());
        self.perform(Operation::Start)
    }

    fn freeze_all(&mut self) -> Result<(), Self::Error> {
        self.perform(Operation::Freeze)
    }

    fn terminate_all_and_cleanup(&mut self) -> Result<(), Self::Error> {
        self.perform(Operation::Terminate)?;
        self.prepared = None;
        Ok(())
    }
}
