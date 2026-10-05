use super::*;
use latch_core::{Action, AgentIdentity, Reversibility, RiskScore};
use latch_policy::{ArgumentMatcher, Effect, Policy, PolicyLevel, Rule, Selector};

pub(super) fn engine(effect: Effect) -> PolicyEngine {
    PolicyEngine::new(vec![Policy {
        id: "default".into(),
        level: PolicyLevel::Company,
        rules: vec![Rule {
            id: "rule".into(),
            description: "rule".into(),
            action: Selector::Any,
            resource: Selector::Any,
            actor: Selector::Any,
            environment: Selector::Any,
            arguments: ArgumentMatcher::Any,
            effect,
        }],
    }])
    .unwrap()
}

pub(super) fn setup() -> (ExecutionGate, Action) {
    let identity = AgentIdentity {
        id: "agent://acme/coder".into(),
        organization: "acme".into(),
        team: "dev".into(),
        owner: "owner".into(),
        purpose: "test".into(),
        provider: "local".into(),
        model: "synthetic".into(),
        model_version: "1".into(),
        runtime: "latch".into(),
        environment: "test".into(),
        device: "synthetic".into(),
        created_at: 10,
        expires_at: 100,
        trust_level: 0,
    };
    let action = Action {
        id: "a1".into(),
        actor: identity.id.clone(),
        session_id: "s1".into(),
        name: "file.write".into(),
        resource: "synthetic".into(),
        environment: "test".into(),
        arguments: vec![],
        reversibility: Reversibility::FullyReversible,
        risk: RiskScore::new(0).unwrap(),
    };
    let session = Session::new("s1".into(), identity, vec!["default".into()], 10).unwrap();
    (ExecutionGate::new(session, engine(Effect::Allow)), action)
}

pub(super) struct UnreachableAdapter;

impl ActionAdapter for UnreachableAdapter {
    type Output = ();
    type Error = ();
    fn costs(&self, _: &Action, _: &BTreeMap<String, u64>) -> Result<BTreeMap<String, u64>, ()> {
        panic!("unauthorized preparation")
    }
    fn execute(&mut self, _: &Action) -> Result<(), ()> {
        panic!("unauthorized execution")
    }
}
