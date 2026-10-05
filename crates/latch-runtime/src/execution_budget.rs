use super::{ActionAdapter, ExecutionError, ExecutionGate};
use crate::budget::BudgetScope;
use latch_audit::{AuditEvent, AuditSink, EventResult};
use latch_core::Action;
use std::collections::BTreeMap;

impl<S: AuditSink> ExecutionGate<S> {
    pub(super) fn prepare_costs<A: ActionAdapter>(
        &mut self,
        action: &Action,
        limits: &BTreeMap<String, u64>,
        now: u64,
        adapter: &A,
    ) -> Result<BTreeMap<String, u64>, ExecutionError<A::Error, S::Error>> {
        match adapter.costs(action, limits) {
            Ok(costs) => Ok(costs),
            Err(error) => {
                self.audit
                    .append(&AuditEvent::for_action(
                        &self.session,
                        action,
                        EventResult::PreparationFailed,
                        now,
                    ))
                    .map_err(ExecutionError::AuditBeforeExecution)?;
                Err(ExecutionError::Adapter(error))
            }
        }
    }

    pub(super) fn charge_execution<E>(
        &mut self,
        action: &Action,
        limits: &BTreeMap<String, u64>,
        costs: &BTreeMap<String, u64>,
        now: u64,
    ) -> Result<(), ExecutionError<E, S::Error>> {
        let scope = BudgetScope::new(
            self.session.identity().organization.clone(),
            self.session.id().into(),
        )
        .map_err(ExecutionError::Budget)?;
        if let Err(error) = self.budgets.charge(&scope, limits, costs) {
            self.audit
                .append(&AuditEvent::for_action(
                    &self.session,
                    action,
                    EventResult::BudgetRejected,
                    now,
                ))
                .map_err(ExecutionError::AuditBeforeExecution)?;
            return Err(ExecutionError::Budget(error));
        }
        Ok(())
    }
}
