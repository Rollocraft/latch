use latch_core::{Action, AgentIdentity, Reversibility, RiskScore, Session};
use serde_json::Value;

use crate::*;

pub(crate) fn identity() -> AgentIdentity {
    AgentIdentity {
        id: "agent://acme/backend/coder".into(),
        organization: "acme".into(),
        team: "backend".into(),
        owner: "operator".into(),
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

pub(crate) fn capability(action: &str, resource: &str, subtree: bool) -> RequestedCapability {
    let resource = ResourceId::new(resource).unwrap();
    RequestedCapability::new(
        ActionName::new(action).unwrap(),
        if subtree {
            ResourceScope::subtree(resource)
        } else {
            ResourceScope::exact(resource)
        },
        "development",
    )
    .unwrap()
}

pub(crate) fn manifest(requires: Vec<RequestedCapability>) -> AgentManifest {
    AgentManifest::new(
        SCHEMA_VERSION,
        AgentDescriptor::new("backend-developer", "1.0.0", None).unwrap(),
        IdentityBinding::from_identity(&identity()).unwrap(),
        requires,
    )
    .unwrap()
}

pub(crate) fn sample() -> AgentManifest {
    manifest(vec![capability("file.read", "file:///project/src", true)])
}

pub(crate) fn input() -> Value {
    serde_json::to_value(sample()).unwrap()
}

pub(crate) fn action() -> Action {
    Action {
        id: "action-1".into(),
        actor: identity().id,
        session_id: "session-1".into(),
        name: "file.read".into(),
        resource: "file:///project/src/lib.rs".into(),
        environment: "development".into(),
        arguments: vec![],
        reversibility: Reversibility::Irreversible,
        risk: RiskScore::new(100).unwrap(),
    }
}

pub(crate) fn session() -> Session {
    Session::new("session-1".into(), identity(), vec!["default".into()], 10).unwrap()
}
