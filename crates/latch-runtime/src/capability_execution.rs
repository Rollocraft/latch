use super::{
    ActionAdapter, CapabilityError, CapabilityExecutionError, CapabilityToken, ExecutionGate,
};
use latch_audit::{AuditEvent, AuditSink, EventResult};
use latch_core::Action;

impl<S: AuditSink> ExecutionGate<S> {
    pub fn execute_with_capability<A: ActionAdapter>(
        &mut self,
        token: &CapabilityToken,
        action: &Action,
        now: u64,
        adapter: &mut A,
    ) -> Result<A::Output, CapabilityExecutionError<A::Error, S::Error>> {
        if let Err(error) = self.validate_capability(token, action, now) {
            let result = if matches!(
                error,
                CapabilityError::Consumed | CapabilityError::AlreadyAttempted
            ) {
                EventResult::ReplayRejected
            } else {
                EventResult::Denied
            };
            self.audit
                .append(&AuditEvent::for_action(&self.session, action, result, now))
                .map_err(|error| {
                    CapabilityExecutionError::Capability(CapabilityError::Audit(error))
                })?;
            return Err(CapabilityExecutionError::Capability(error));
        }
        self.consume_capabilities(&action.id);
        self.attempted.insert(action.id.clone());
        self.execute_inner(action, now, adapter, true)
            .map_err(CapabilityExecutionError::Execution)
    }
}
