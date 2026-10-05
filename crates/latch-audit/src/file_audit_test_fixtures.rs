use crate::{AuditEvent, EventResult};
use latch_core::RiskScore;

pub(super) fn temporary(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "latch-audit-{label}-{}-{}.bin",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

pub(super) fn event(action_id: &str) -> AuditEvent {
    AuditEvent {
        timestamp: 42,
        organization: "acme".into(),
        owner: "owner".into(),
        agent: "agent://acme/coder".into(),
        session: "s1".into(),
        action_id: action_id.into(),
        action: "file.read".into(),
        resource: "project/readme".into(),
        environment: "dev".into(),
        risk: RiskScore::new(2).unwrap(),
        result: EventResult::Succeeded,
        policies: vec!["default".into()],
    }
}
