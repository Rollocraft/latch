use super::ResourceExportPolicy;
use crate::{AuditEvent, EventResult};
use serde::Serialize;

fn result_name(result: EventResult) -> &'static str {
    match result {
        EventResult::Denied => "denied",
        EventResult::ApprovalRequired => "approval_required",
        EventResult::Authorized => "authorized",
        EventResult::Started => "started",
        EventResult::Succeeded => "succeeded",
        EventResult::Failed => "failed",
        EventResult::ReplayRejected => "replay_rejected",
        EventResult::BudgetRejected => "budget_rejected",
        EventResult::PreparationFailed => "preparation_failed",
    }
}

#[derive(Serialize)]
pub(super) struct ExportRecord<'a> {
    timestamp: u64,
    organization: &'a str,
    owner: &'a str,
    agent: &'a str,
    session: &'a str,
    action_id: &'a str,
    action: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<&'a str>,
    environment: &'a str,
    risk: u8,
    result: &'static str,
}

impl<'a> ExportRecord<'a> {
    pub(super) fn new(event: &'a AuditEvent, policy: ResourceExportPolicy) -> Self {
        let resource = match policy {
            ResourceExportPolicy::Omit => None,
            ResourceExportPolicy::Redact => Some("[REDACTED]"),
            ResourceExportPolicy::Include => Some(event.resource.as_str()),
        };
        Self {
            timestamp: event.timestamp,
            organization: &event.organization,
            owner: &event.owner,
            agent: &event.agent,
            session: &event.session,
            action_id: &event.action_id,
            action: &event.action,
            resource,
            environment: &event.environment,
            risk: event.risk.value(),
            result: result_name(event.result),
        }
    }
}
