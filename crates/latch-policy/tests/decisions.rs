use latch_core::{Action, AgentIdentity, Reversibility, RiskScore, Session};
use latch_policy::*;

fn request() -> (Session, Action) {
    let identity = AgentIdentity {
        id: "agent://acme/coder".into(),
        organization: "acme".into(),
        team: "backend".into(),
        owner: "david".into(),
        purpose: "coding".into(),
        provider: "local".into(),
        model: "coder".into(),
        model_version: "1".into(),
        runtime: "latch".into(),
        environment: "production".into(),
        device: "workstation".into(),
        created_at: 10,
        expires_at: 100,
        trust_level: 100,
    };
    let action = Action {
        id: "a1".into(),
        actor: identity.id.clone(),
        session_id: "s1".into(),
        name: "git.push".into(),
        resource: "repo/main".into(),
        environment: "production".into(),
        arguments: vec!["push".into(), "origin".into(), "main".into()],
        reversibility: Reversibility::Irreversible,
        risk: RiskScore::new(65).unwrap(),
    };
    (
        Session::new("s1".into(), identity, vec!["company".into()], 10).unwrap(),
        action,
    )
}

fn rule(id: &str, effect: Effect) -> Rule {
    Rule {
        id: id.into(),
        description: format!("Rule {id}"),
        action: Selector::Exact("git.push".into()),
        resource: Selector::Any,
        environment: Selector::Any,
        actor: Selector::Any,
        arguments: ArgumentMatcher::Any,
        effect,
    }
}

fn policy(id: &str, level: PolicyLevel, rules: Vec<Rule>) -> Policy {
    Policy {
        id: id.into(),
        level,
        rules,
    }
}

#[test]
fn company_deny_survives_session_allow_and_high_trust() {
    let (session, action) = request();
    let policies = vec![
        policy(
            "session",
            PolicyLevel::Session,
            vec![rule("allow", Effect::Allow)],
        ),
        policy(
            "company",
            PolicyLevel::Company,
            vec![rule("deny", Effect::Deny)],
        ),
    ];
    let first = PolicyEngine::new(policies.clone())
        .unwrap()
        .evaluate(&session, &action, 20);
    let reversed = PolicyEngine::new(policies.into_iter().rev().collect())
        .unwrap()
        .evaluate(&session, &action, 20);
    assert_eq!(first, reversed);
    assert_eq!(first.outcome, Outcome::Deny);
    assert_eq!(first.reason, Reason::ExplicitDeny);
    assert_eq!(first.matches[0].policy, "company");
    assert_eq!(first.matches[0].description, "Rule deny");
}

#[test]
fn missing_policy_and_unmatched_actions_fail_closed() {
    let (session, mut action) = request();
    let absent = PolicyEngine::new(vec![]).unwrap();
    assert_eq!(
        absent.evaluate(&session, &action, 20).reason,
        Reason::MissingPolicy("company".into())
    );
    let engine = PolicyEngine::new(vec![policy(
        "company",
        PolicyLevel::Company,
        vec![rule("allow", Effect::Allow)],
    )])
    .unwrap();
    action.name = "database.drop".into();
    assert_eq!(
        engine.evaluate(&session, &action, 20).reason,
        Reason::DefaultDeny
    );
    action.session_id = "forged".into();
    assert!(matches!(
        engine.evaluate(&session, &action, 20).reason,
        Reason::InvalidAction(_)
    ));
}

#[test]
fn approval_retains_strictest_budget_and_all_explanations() {
    let (session, action) = request();
    let engine = PolicyEngine::new(vec![policy(
        "company",
        PolicyLevel::Company,
        vec![
            rule("allow", Effect::Allow),
            rule("ask", Effect::Ask),
            rule(
                "small",
                Effect::Limit {
                    budget: "pushes".into(),
                    maximum: 2,
                },
            ),
            rule(
                "large",
                Effect::Limit {
                    budget: "pushes".into(),
                    maximum: 10,
                },
            ),
            rule("log", Effect::Log),
        ],
    )])
    .unwrap();
    let decision = engine.evaluate(&session, &action, 20);
    assert_eq!(decision.outcome, Outcome::ApprovalRequired);
    assert_eq!(decision.limits["pushes"], 2);
    assert_eq!(decision.matches.len(), 5);
}

#[test]
fn logging_and_limits_do_not_grant_permission() {
    let (session, action) = request();
    for effect in [
        Effect::Log,
        Effect::Limit {
            budget: "pushes".into(),
            maximum: 1,
        },
    ] {
        let engine = PolicyEngine::new(vec![policy(
            "company",
            PolicyLevel::Company,
            vec![rule("observe", effect)],
        )])
        .unwrap();
        assert_eq!(
            engine.evaluate(&session, &action, 20).outcome,
            Outcome::Deny
        );
    }
}

#[test]
fn argument_allow_does_not_cover_force_push_or_extra_arguments() {
    let (session, mut action) = request();
    let mut allow = rule("ordinary-push", Effect::Allow);
    allow.arguments = ArgumentMatcher::Exactly(action.arguments.clone());
    let engine =
        PolicyEngine::new(vec![policy("company", PolicyLevel::Company, vec![allow])]).unwrap();
    assert_eq!(
        engine.evaluate(&session, &action, 20).outcome,
        Outcome::Allow
    );
    action.arguments.push("--force".into());
    assert_eq!(
        engine.evaluate(&session, &action, 20).outcome,
        Outcome::Deny
    );
}

/// The distinction argument-aware policies exist for: the same action, allowed
/// or refused by how it was invoked.
#[test]
fn a_force_push_is_denied_while_an_ordinary_push_is_allowed() {
    let (session, action) = request();
    let mut allow = rule("push", Effect::Allow);
    allow.arguments = ArgumentMatcher::Positional {
        index: 0,
        value: "push".into(),
    };
    let mut deny = rule("no-force", Effect::Deny);
    deny.arguments = ArgumentMatcher::AnyFlag(vec![
        "-f".into(),
        "--force".into(),
        "--force-with-lease".into(),
    ]);
    let engine = PolicyEngine::new(vec![policy(
        "company",
        PolicyLevel::Company,
        vec![allow, deny],
    )])
    .unwrap();

    assert_eq!(
        engine.evaluate(&session, &action, 20).outcome,
        Outcome::Allow
    );
    for forced in [
        vec!["push", "--force"],
        vec!["push", "-f"],
        vec!["push", "-qf"],
    ] {
        let mut action = action.clone();
        action.arguments = forced.iter().map(|a| (*a).to_string()).collect();
        let decision = engine.evaluate(&session, &action, 20);
        assert_eq!(decision.outcome, Outcome::Deny, "{forced:?}");
        assert_eq!(decision.reason, Reason::ExplicitDeny);
    }
}

/// A rule is only as good as its own validation: a matcher that could never
/// mean anything is rejected when the policy is built, not silently ignored
/// while the rule appears to be in force.
#[test]
fn a_rule_with_an_unusable_matcher_is_refused() {
    let mut broken = rule("bad", Effect::Deny);
    broken.arguments = ArgumentMatcher::Flag("force".into());
    assert!(matches!(
        PolicyEngine::new(vec![policy("company", PolicyLevel::Company, vec![broken])]),
        Err(PolicyError::InvalidPolicy)
    ));
}

#[test]
fn granted_actions_with_budgets_require_enforcement() {
    let (session, action) = request();
    let engine = PolicyEngine::new(vec![policy(
        "company",
        PolicyLevel::Company,
        vec![
            rule("allow", Effect::Allow),
            rule(
                "budget",
                Effect::Limit {
                    budget: "pushes".into(),
                    maximum: 0,
                },
            ),
        ],
    )])
    .unwrap();
    assert_eq!(
        engine.evaluate(&session, &action, 20).outcome,
        Outcome::Limited
    );
}

#[test]
fn ambiguous_policy_and_rule_ids_are_rejected() {
    let p = policy("company", PolicyLevel::Company, vec![]);
    assert_eq!(
        PolicyEngine::new(vec![p.clone(), p]).unwrap_err(),
        PolicyError::DuplicatePolicy
    );
    let r = rule("same", Effect::Allow);
    assert_eq!(
        PolicyEngine::new(vec![policy(
            "company",
            PolicyLevel::Company,
            vec![r.clone(), r]
        )])
        .unwrap_err(),
        PolicyError::DuplicateRule
    );
}
