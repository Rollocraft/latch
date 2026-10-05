use crate::*;
use latch_core::{Action, AgentIdentity, Reversibility, RiskScore, Session};
use latch_policy::{
    ArgumentMatcher, Decision, Effect, Policy, PolicyEngine, PolicyLevel, Rule, Selector,
};

pub(crate) fn session() -> Session {
    Session::new(
        "session-1".into(),
        AgentIdentity {
            id: "agent://acme/coder".into(),
            organization: "acme".into(),
            team: "backend".into(),
            owner: "human-owner".into(),
            purpose: "coding".into(),
            provider: "local".into(),
            model: "coder".into(),
            model_version: "1".into(),
            runtime: "latch".into(),
            environment: "production".into(),
            device: "workstation".into(),
            created_at: 10,
            expires_at: 100,
            trust_level: 0,
        },
        vec!["production-policy".into()],
        10,
    )
    .unwrap()
}

pub(crate) fn action() -> Action {
    Action {
        id: "action-1".into(),
        actor: session().identity().id.clone(),
        session_id: session().id().into(),
        name: "git.push".into(),
        resource: "repo/main".into(),
        environment: "production".into(),
        arguments: vec![
            "SECRET_TOKEN=do-not-print".into(),
            "payload-contents".into(),
        ],
        reversibility: Reversibility::Irreversible,
        risk: RiskScore::new(85).unwrap(),
    }
}

pub(crate) fn decision() -> Decision {
    PolicyEngine::new(vec![Policy {
        id: "production-policy".into(),
        level: PolicyLevel::Company,
        rules: vec![Rule {
            id: "review-push".into(),
            description: "Production branch requires human review".into(),
            action: Selector::Any,
            resource: Selector::Any,
            environment: Selector::Any,
            actor: Selector::Any,
            arguments: ArgumentMatcher::Any,
            effect: Effect::Ask,
        }],
    }])
    .unwrap()
    .evaluate(&session(), &action(), 20)
}

pub(crate) fn assert_safe(text: &str) {
    assert!(text.len() <= MAX_OUTPUT_BYTES);
    assert!(!text.chars().any(|c| c.is_control() && c != '\n'));
    for c in [
        '\u{202e}', '\u{2066}', '\u{2069}', '\u{2028}', '\u{2029}', '\u{200b}',
    ] {
        assert!(!text.contains(c));
    }
}
