use crate::{AuditEvent, EventResult};
use latch_core::RiskScore;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct ReportCounts {
    pub events: usize,
    pub denials: usize,
    pub approval_required: usize,
    pub authorizations: usize,
    pub high_risk_events: usize,
}

impl ReportCounts {
    pub(super) fn record(&mut self, event: &AuditEvent, high_risk: RiskScore) {
        self.events += 1;
        self.denials += usize::from(matches!(
            event.result,
            EventResult::Denied | EventResult::ReplayRejected | EventResult::BudgetRejected
        ));
        self.approval_required += usize::from(event.result == EventResult::ApprovalRequired);
        self.authorizations += usize::from(event.result == EventResult::Authorized);
        self.high_risk_events += usize::from(event.risk >= high_risk);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionCounts {
    pub action: String,
    pub counts: ReportCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComplianceReport {
    pub organization: String,
    pub high_risk_threshold: u8,
    pub totals: ReportCounts,
    pub actions: Vec<ActionCounts>,
    pub total_action_names: usize,
    pub next_offset: Option<usize>,
}
