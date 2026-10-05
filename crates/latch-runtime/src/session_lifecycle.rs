use super::ExecutionGate;
use crate::budget::BudgetLedger;
use latch_audit::{AuditSink, MemoryAudit};
use latch_core::{Session, SessionState, ValidationError};
use latch_policy::PolicyEngine;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

impl ExecutionGate {
    pub fn new(session: Session, policies: PolicyEngine) -> Self {
        Self::with_audit(session, policies, MemoryAudit::default())
    }
}

impl<S: AuditSink> ExecutionGate<S> {
    pub fn with_audit(session: Session, policies: PolicyEngine, audit: S) -> Self {
        Self {
            session,
            policies,
            budgets: BudgetLedger::new(),
            attempted: BTreeSet::new(),
            approvals: BTreeMap::new(),
            approval_history: Vec::new(),
            approval_quorum: None,
            capabilities: BTreeMap::new(),
            next_capability_id: 1,
            pending_capability_audit: VecDeque::new(),
            audit,
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn audit(&self) -> &S {
        &self.audit
    }

    pub fn transition(&mut self, next: SessionState, now: u64) -> Result<(), ValidationError> {
        self.session.transition(next, now)?;
        if next != SessionState::Active {
            self.revoke_unused_approvals();
            self.revoke_unused_capabilities(now);
        }
        let _ = self.flush_capability_audit();
        Ok(())
    }

    fn revoke_unused_approvals(&mut self) {
        for approval in self.approvals.values_mut().flatten() {
            if !approval.consumed {
                approval.revoked = true;
            }
        }
    }

    fn revoke_unused_capabilities(&mut self, now: u64) {
        let mut revoked = Vec::new();
        for record in self.capabilities.values_mut() {
            if !record.consumed && !record.revoked {
                record.revoked = true;
                revoked.push(record.clone());
            }
        }
        for record in revoked {
            self.queue_capability_revocation(&record, now);
        }
    }
}
