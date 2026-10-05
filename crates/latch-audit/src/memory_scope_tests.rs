use crate::*;
use latch_core::RiskScore;

#[test]
fn session_queries_require_matching_organization() {
    let mut audit = MemoryAudit::default();
    let event = AuditEvent {
        timestamp: 1,
        organization: "acme".into(),
        owner: "owner".into(),
        agent: "agent://acme/coder".into(),
        session: "s1".into(),
        action_id: "a1".into(),
        action: "file.read".into(),
        resource: "project/readme".into(),
        environment: "dev".into(),
        risk: RiskScore::new(2).unwrap(),
        result: EventResult::Succeeded,
        policies: vec!["default".into()],
    };
    audit.append(&event).unwrap();
    assert_eq!(audit.for_session("acme", "s1").count(), 1);
    assert_eq!(audit.for_session("other", "s1").count(), 0);
    assert_eq!(audit.for_session("acme", "s2").count(), 0);
    assert_eq!(audit.events(), &[event]);
}
