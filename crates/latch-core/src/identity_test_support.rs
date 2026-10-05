use super::*;

pub(super) fn identity() -> AgentIdentity {
    AgentIdentity {
        id: "agent://acme/backend/coder".into(),
        organization: "acme".into(),
        team: "backend".into(),
        owner: "david".into(),
        purpose: "coding".into(),
        provider: "local".into(),
        model: "coder".into(),
        model_version: "1".into(),
        runtime: "latch".into(),
        environment: "development".into(),
        device: "workstation".into(),
        created_at: 10,
        expires_at: 100,
        trust_level: 0,
    }
}
pub(super) fn session() -> Session {
    Session::new("s1".into(), identity(), vec!["default".into()], 10).unwrap()
}
pub(super) fn action() -> Action {
    Action {
        id: "a1".into(),
        session_id: "s1".into(),
        actor: identity().id,
        name: "git.push".into(),
        resource: "repo/main".into(),
        environment: "development".into(),
        arguments: vec![],
        reversibility: Reversibility::Irreversible,
        risk: RiskScore::new(65).unwrap(),
    }
}
