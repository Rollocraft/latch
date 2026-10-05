use super::page_request::next_offset;
use super::{ActionCounts, AuditQuery, AuditReader, ComplianceReport, PageRequest, ReportCounts};
use latch_core::RiskScore;
use std::collections::BTreeMap;

impl AuditReader<'_> {
    pub fn report(
        &self,
        query: &AuditQuery,
        page: PageRequest,
        high_risk: RiskScore,
    ) -> ComplianceReport {
        let mut totals = ReportCounts::default();
        let mut actions: BTreeMap<&str, ReportCounts> = BTreeMap::new();
        for event in self.events.iter().filter(|event| query.matches(event)) {
            totals.record(event, high_risk);
            actions
                .entry(&event.action)
                .or_default()
                .record(event, high_risk);
        }
        let total_action_names = actions.len();
        ComplianceReport {
            organization: query.organization().into(),
            high_risk_threshold: high_risk.value(),
            totals,
            actions: actions
                .into_iter()
                .skip(page.offset())
                .take(page.limit())
                .map(|(action, counts)| ActionCounts {
                    action: action.into(),
                    counts,
                })
                .collect(),
            total_action_names,
            next_offset: next_offset(page, total_action_names),
        }
    }
}
