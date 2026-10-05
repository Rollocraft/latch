use super::{ApprovalQuorum, ApprovalRecord, ExecutionGate};
use latch_audit::AuditSink;
use latch_core::{Action, Session};
use latch_policy::PolicyEngine;
use std::collections::BTreeSet;

/// A pending approval set counts only when enough distinct, still-valid,
/// unconsumed and unrevoked votes cover the exact action. Without a quorum one
/// distinct vote suffices, preserving the previous single-approval behavior.
pub(super) fn has_approval_records(
    records: Option<&Vec<ApprovalRecord>>,
    quorum: Option<&ApprovalQuorum>,
    action: &Action,
    now: u64,
) -> bool {
    let required = quorum.map_or(1, ApprovalQuorum::required);
    let Some(records) = records else {
        return false;
    };
    let distinct: BTreeSet<&str> = records
        .iter()
        .filter(|approval| {
            !approval.consumed
                && !approval.revoked
                && approval.action == *action
                && approval.issued_at <= now
                && now < approval.expires_at
        })
        .map(|approval| approval.approver.as_str())
        .collect();
    distinct.len() >= required
}

impl<S: AuditSink> ExecutionGate<S> {
    /// Configure the quorum at construction; it cannot be weakened after grants.
    /// Only ASK decisions require it; DENY, limits and identity checks still apply.
    pub fn with_approval_quorum(
        session: Session,
        policies: PolicyEngine,
        audit: S,
        quorum: ApprovalQuorum,
    ) -> Self {
        let mut gate = Self::with_audit(session, policies, audit);
        gate.approval_quorum = Some(quorum);
        gate
    }

    pub fn approvals(&self) -> impl Iterator<Item = &ApprovalRecord> {
        self.approval_history
            .iter()
            .chain(self.approvals.values().flatten())
    }

    /// Revoke all unused votes for an action, retaining attribution records.
    /// This operation belongs to the authenticated control-plane interface.
    pub fn revoke_approval(&mut self, action_id: &str) -> bool {
        let mut revoked = false;
        if let Some(records) = self.approvals.get_mut(action_id) {
            for record in records {
                if !record.consumed && !record.revoked {
                    record.revoked = true;
                    revoked = true;
                }
            }
        }
        revoked
    }
}
