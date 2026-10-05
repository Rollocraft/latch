use super::{ApprovalError, ApprovalRecord, ExecutionGate};
use latch_audit::{AuditEvent, AuditSink, EventResult};
use latch_core::{Action, Session};
use latch_policy::Outcome;

fn approval_event(
    session: &Session,
    action: &Action,
    approver: &str,
    operation: &str,
    result: EventResult,
    now: u64,
) -> AuditEvent {
    let mut event = AuditEvent::for_action(session, action, result, now);
    event.action = operation.into();
    event.resource = format!("approval/{}", approver);
    event
}

impl<S: AuditSink> ExecutionGate<S> {
    /// The caller must authenticate and authorize the approver before calling.
    /// Records are process-local and are not bearer credentials. A grant that
    /// cannot be audited is not granted: the record is discarded fail-closed.
    pub fn approve_once(
        &mut self,
        action: &Action,
        approver: String,
        now: u64,
        expires_at: u64,
    ) -> Result<(), ApprovalError> {
        if approver.trim().is_empty() || approver.chars().any(char::is_control) {
            return Err(ApprovalError::InvalidApprover);
        }
        if self
            .approval_quorum
            .as_ref()
            .is_some_and(|q| !q.permits(&approver))
        {
            return Err(ApprovalError::UnauthorizedApprover);
        }
        if expires_at <= now || expires_at > self.session.identity().expires_at {
            return Err(ApprovalError::InvalidExpiration);
        }
        if self.attempted.contains(&action.id)
            || self.policies.evaluate(&self.session, action, now).outcome
                != Outcome::ApprovalRequired
        {
            return Err(ApprovalError::NotPending);
        }
        let records = self.approvals.entry(action.id.clone()).or_default();
        if records.iter().any(|record| record.action != *action) {
            return Err(ApprovalError::ActionChanged);
        }
        // Without a quorum a gate holds one approval, which a new approval replaces.
        let previous = records
            .iter()
            .position(|record| self.approval_quorum.is_none() || record.approver == approver);
        if let Some(index) = previous {
            let record = &records[index];
            if record.consumed
                || now < record.issued_at
                || (!record.revoked && now < record.expires_at)
            {
                return Err(ApprovalError::AlreadyApproved);
            }
            self.approval_history.push(records.remove(index));
        }
        let event = approval_event(
            &self.session,
            action,
            &approver,
            "runtime.approval.grant",
            EventResult::Authorized,
            now,
        );
        if self.audit.append(&event).is_err() {
            return Err(ApprovalError::ApproveAuditFailed);
        }
        records.push(ApprovalRecord {
            action: action.clone(),
            approver,
            issued_at: now,
            expires_at,
            consumed: false,
            revoked: false,
        });
        Ok(())
    }
}
