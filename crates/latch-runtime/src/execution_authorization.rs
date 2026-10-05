use super::{CapabilityError, ExecutionError, ExecutionGate};
use latch_audit::{AuditEvent, AuditSink, EventResult};
use latch_core::Action;
use latch_policy::{Decision, Outcome};

fn refusal_result(decision: &Decision) -> EventResult {
    if decision.outcome == Outcome::ApprovalRequired {
        EventResult::ApprovalRequired
    } else {
        EventResult::Denied
    }
}

impl<S: AuditSink> ExecutionGate<S> {
    fn permits_decision(&self, decision: &Decision, action: &Action, now: u64) -> bool {
        matches!(decision.outcome, Outcome::Allow | Outcome::Limited)
            || (decision.outcome == Outcome::ApprovalRequired && self.has_approval(action, now))
    }

    pub(super) fn has_approval(&self, action: &Action, now: u64) -> bool {
        super::approval_lifecycle::has_approval_records(
            self.approvals.get(&action.id),
            self.approval_quorum.as_ref(),
            action,
            now,
        )
    }

    pub(super) fn authorize_capability(
        &mut self,
        action: &Action,
        now: u64,
    ) -> Result<(), CapabilityError<S::Error>> {
        let decision = self.policies.evaluate(&self.session, action, now);
        if !self.permits_decision(&decision, action, now) {
            self.audit
                .append(&AuditEvent::for_action(
                    &self.session,
                    action,
                    refusal_result(&decision),
                    now,
                ))
                .map_err(CapabilityError::Audit)?;
            return Err(CapabilityError::Policy(Box::new(decision)));
        }
        Ok(())
    }

    pub(super) fn authorize_execution<E>(
        &mut self,
        action: &Action,
        now: u64,
        capability_attempt: bool,
    ) -> Result<(Decision, bool), ExecutionError<E, S::Error>> {
        let decision = self.policies.evaluate(&self.session, action, now);
        let approved =
            decision.outcome == Outcome::ApprovalRequired && self.has_approval(action, now);
        if !self.permits_decision(&decision, action, now) {
            self.audit
                .append(&AuditEvent::for_action(
                    &self.session,
                    action,
                    refusal_result(&decision),
                    now,
                ))
                .map_err(ExecutionError::AuditBeforeExecution)?;
            return Err(ExecutionError::Policy(Box::new(decision)));
        }
        if !capability_attempt && self.attempted.contains(&action.id) {
            self.audit
                .append(&AuditEvent::for_action(
                    &self.session,
                    action,
                    EventResult::ReplayRejected,
                    now,
                ))
                .map_err(ExecutionError::AuditBeforeExecution)?;
            return Err(ExecutionError::AlreadyAttempted);
        }
        Ok((decision, approved))
    }
}
