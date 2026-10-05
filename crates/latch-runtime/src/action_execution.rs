use super::{ActionAdapter, ExecutionError, ExecutionGate};
use latch_audit::{AuditEvent, AuditSink, EventResult};
use latch_core::Action;

impl<S: AuditSink> ExecutionGate<S> {
    pub fn execute<A: ActionAdapter>(
        &mut self,
        action: &Action,
        now: u64,
        adapter: &mut A,
    ) -> Result<A::Output, ExecutionError<A::Error, S::Error>> {
        self.execute_inner(action, now, adapter, false)
    }

    pub(super) fn execute_inner<A: ActionAdapter>(
        &mut self,
        action: &Action,
        now: u64,
        adapter: &mut A,
        capability_attempt: bool,
    ) -> Result<A::Output, ExecutionError<A::Error, S::Error>> {
        self.flush_capability_audit()
            .map_err(ExecutionError::AuditBeforeExecution)?;
        let (decision, approved) = self.authorize_execution(action, now, capability_attempt)?;
        let costs = self.prepare_costs(action, &decision.limits, now, adapter)?;
        self.charge_execution(action, &decision.limits, &costs, now)?;
        self.consume_execution_grants(action, approved);
        self.run_adapter(action, now, adapter)
    }

    fn consume_execution_grants(&mut self, action: &Action, approved: bool) {
        self.attempted.insert(action.id.clone());
        self.consume_capabilities(&action.id);
        if approved {
            for approval in self
                .approvals
                .get_mut(&action.id)
                .expect("validated approval")
            {
                approval.consumed = true;
            }
        }
    }

    fn run_adapter<A: ActionAdapter>(
        &mut self,
        action: &Action,
        now: u64,
        adapter: &mut A,
    ) -> Result<A::Output, ExecutionError<A::Error, S::Error>> {
        self.audit
            .append(&AuditEvent::for_action(
                &self.session,
                action,
                EventResult::Started,
                now,
            ))
            .map_err(ExecutionError::AuditBeforeExecution)?;
        let result = adapter.execute(action);
        let status = if result.is_ok() {
            EventResult::Succeeded
        } else {
            EventResult::Failed
        };
        self.audit
            .append(&AuditEvent::for_action(&self.session, action, status, now))
            .map_err(ExecutionError::AuditAfterExecution)?;
        result.map_err(ExecutionError::Adapter)
    }
}
