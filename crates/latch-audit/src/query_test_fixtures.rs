use crate::*;
use latch_core::RiskScore;
use serde_json::Value;

pub(super) fn event(organization: &str, timestamp: u64, result: EventResult) -> AuditEvent {
    AuditEvent {
        timestamp,
        organization: organization.into(),
        owner: "owner".into(),
        agent: "shared-agent".into(),
        session: "shared-session".into(),
        action_id: "shared-action".into(),
        action: "file.read".into(),
        resource: "private/resource?token=secret".into(),
        environment: "dev".into(),
        risk: risk(70),
        result,
        policies: vec!["private-policy".into()],
    }
}

pub(super) fn risk(value: u8) -> RiskScore {
    RiskScore::new(value).unwrap()
}

pub(super) fn query(organization: &str) -> AuditQuery {
    AuditQuery::new(organization, AuditFilters::default()).unwrap()
}

pub(super) fn page(offset: usize, limit: usize) -> PageRequest {
    PageRequest::new(offset, limit).unwrap()
}

pub(super) fn export(
    reader: &AuditReader<'_>,
    query: &AuditQuery,
    policy: ResourceExportPolicy,
) -> Vec<Value> {
    let mut bytes = Vec::new();
    reader
        .export_jsonl(query, page(0, MAX_PAGE_SIZE), policy, &mut bytes)
        .unwrap();
    String::from_utf8(bytes)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
