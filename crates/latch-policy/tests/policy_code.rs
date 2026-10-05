use latch_core::{Action, AgentIdentity, Reversibility, RiskScore, Session};
use latch_policy::*;
use serde_json::{Value, json};

fn rule(effect: Value) -> Value {
    json!({
        "id": "read",
        "description": "Synthetic item access",
        "action": {"kind": "exact", "value": "item.read"},
        "resource": {"kind": "prefix", "value": "items/"},
        "environment": {"kind": "exact", "value": "test"},
        "actor": {"kind": "any"},
        "effect": effect
    })
}

fn document(rule: Value) -> Value {
    json!({"version": 1, "policies": [{"id": "base", "level": "company", "rules": [rule]}]})
}

fn parse(value: &Value, format: PolicyFormat) -> Result<PolicyDocument, PolicyLoadError> {
    let input = match format {
        PolicyFormat::Json => serde_json::to_vec(value).unwrap(),
        PolicyFormat::Yaml => serde_yaml::to_string(value).unwrap().into_bytes(),
    };
    load_policy_document(&input, format)
}

fn request() -> (Session, Action) {
    let identity = AgentIdentity {
        id: "agent://example/tester".into(),
        organization: "example".into(),
        team: "testing".into(),
        owner: "tester".into(),
        purpose: "policy tests".into(),
        provider: "local".into(),
        model: "synthetic".into(),
        model_version: "1".into(),
        runtime: "latch".into(),
        environment: "test".into(),
        device: "test-device".into(),
        created_at: 10,
        expires_at: 100,
        trust_level: 0,
    };
    let action = Action {
        id: "read-1".into(),
        actor: identity.id.clone(),
        session_id: "test-session".into(),
        name: "item.read".into(),
        resource: "items/example".into(),
        environment: "test".into(),
        arguments: vec!["sample".into(), "--verbose".into()],
        reversibility: Reversibility::FullyReversible,
        risk: RiskScore::new(0).unwrap(),
    };
    let session = Session::new("test-session".into(), identity, vec!["base".into()], 10).unwrap();
    (session, action)
}

#[test]
fn json_and_yaml_parse_every_effect_and_matcher() {
    let effects = [
        (json!({"kind": "allow"}), Effect::Allow),
        (json!({"kind": "deny"}), Effect::Deny),
        (json!({"kind": "ask"}), Effect::Ask),
        (json!({"kind": "log"}), Effect::Log),
        (
            json!({"kind": "limit", "budget": "reads", "maximum": 0}),
            Effect::Limit {
                budget: "reads".into(),
                maximum: 0,
            },
        ),
    ];
    let matchers = [
        (json!({"kind": "any"}), ArgumentMatcher::Any),
        (json!({"kind": "empty"}), ArgumentMatcher::Empty),
        (
            json!({"kind": "exactly", "values": ["sample", "--verbose"]}),
            ArgumentMatcher::Exactly(vec!["sample".into(), "--verbose".into()]),
        ),
        (
            json!({"kind": "flag", "value": "--verbose"}),
            ArgumentMatcher::Flag("--verbose".into()),
        ),
        (
            json!({"kind": "any_flag", "values": ["-v", "--verbose"]}),
            ArgumentMatcher::AnyFlag(vec!["-v".into(), "--verbose".into()]),
        ),
        (
            json!({"kind": "contains", "value": "sample"}),
            ArgumentMatcher::Contains("sample".into()),
        ),
        (
            json!({"kind": "positional", "index": 0, "value": "sample"}),
            ArgumentMatcher::Positional {
                index: 0,
                value: "sample".into(),
            },
        ),
        (
            json!({"kind": "all", "matchers": [{"kind": "contains", "value": "sample"}, {"kind": "flag", "value": "--verbose"}]}),
            ArgumentMatcher::All(vec![
                ArgumentMatcher::Contains("sample".into()),
                ArgumentMatcher::Flag("--verbose".into()),
            ]),
        ),
    ];
    for format in [PolicyFormat::Json, PolicyFormat::Yaml] {
        for (wire_effect, effect) in &effects {
            for (wire_matcher, matcher) in &matchers {
                let mut wire_rule = rule(wire_effect.clone());
                wire_rule["arguments"] = wire_matcher.clone();
                let loaded = parse(&document(wire_rule), format).unwrap();
                assert_eq!(loaded.version(), POLICY_DOCUMENT_VERSION);
                let parsed = &loaded.policies()[0].rules[0];
                assert_eq!(&parsed.effect, effect);
                assert_eq!(&parsed.arguments, matcher);
                assert_eq!(parsed.action, Selector::Exact("item.read".into()));
                assert_eq!(parsed.resource, Selector::Prefix("items/".into()));
                assert_eq!(parsed.actor, Selector::Any);
                let (session, action) = request();
                let decision = loaded.into_engine().evaluate(&session, &action, 20);
                assert_eq!(
                    decision.matches.len(),
                    usize::from(matcher.matches(&action.arguments))
                );
            }
        }
        let loaded = parse(&document(rule(json!({"kind": "allow"}))), format).unwrap();
        assert_eq!(
            loaded.policies()[0].rules[0].arguments,
            ArgumentMatcher::Any
        );
        let (session, action) = request();
        assert_eq!(
            loaded.into_engine().evaluate(&session, &action, 20).outcome,
            Outcome::Allow
        );
    }
}

#[test]
fn invalid_fields_types_selectors_and_matchers_are_rejected() {
    let valid = document(rule(json!({"kind": "allow"})));
    let mut invalid = Vec::new();
    for path in [
        "",
        "/policies/0",
        "/policies/0/rules/0",
        "/policies/0/rules/0/action",
        "/policies/0/rules/0/actor",
        "/policies/0/rules/0/effect",
    ] {
        let mut value = valid.clone();
        value.pointer_mut(path).unwrap()["unexpected"] = json!(true);
        invalid.push(value);
    }
    for field in [
        "id",
        "description",
        "action",
        "resource",
        "environment",
        "actor",
        "effect",
    ] {
        let mut value = valid.clone();
        value["policies"][0]["rules"][0]
            .as_object_mut()
            .unwrap()
            .remove(field);
        invalid.push(value);
    }
    for (path, replacement) in [
        ("/version", json!("1")),
        ("/version", json!(1.0)),
        ("/version", json!(-1)),
        ("/policies", Value::Null),
        ("/policies/0/level", json!("global")),
        ("/policies/0/id", json!("\nbase")),
        ("/policies/0/rules/0/id", json!("read\n")),
        ("/policies/0/rules/0/description", json!(" \t")),
        (
            "/policies/0/rules/0/action",
            json!({"kind": "glob", "value": "*"}),
        ),
        (
            "/policies/0/rules/0/action",
            json!({"kind": "exact", "value": ""}),
        ),
        (
            "/policies/0/rules/0/resource",
            json!({"kind": "prefix", "value": " "}),
        ),
        (
            "/policies/0/rules/0/actor",
            json!({"kind": "any", "value": "unused"}),
        ),
        (
            "/policies/0/rules/0/effect",
            json!({"kind": "limit", "budget": "reads\n", "maximum": 1}),
        ),
        (
            "/policies/0/rules/0/effect",
            json!({"kind": "limit", "budget": "reads", "maximum": -1}),
        ),
        (
            "/policies/0/rules/0/effect",
            json!({"kind": "limit", "budget": "reads"}),
        ),
        ("/policies/0/rules/0/effect", json!("allow")),
    ] {
        let mut value = valid.clone();
        *value.pointer_mut(path).unwrap() = replacement;
        invalid.push(value);
    }
    for matcher in [
        Value::Null,
        json!({"kind": "any", "value": "unused"}),
        json!({"kind": "empty", "values": []}),
        json!({"kind": "flag", "value": "verbose"}),
        json!({"kind": "flag", "value": "--"}),
        json!({"kind": "flag", "value": "--bad flag"}),
        json!({"kind": "any_flag", "values": []}),
        json!({"kind": "exactly", "values": ["bad\nvalue"]}),
        json!({"kind": "contains", "value": ""}),
        json!({"kind": "positional", "index": -1, "value": "sample"}),
        json!({"kind": "all", "matchers": []}),
        json!({"kind": "all", "matchers": [{"kind": "any", "unknown": true}]}),
    ] {
        let mut value = valid.clone();
        value["policies"][0]["rules"][0]["arguments"] = matcher;
        invalid.push(value);
    }
    for format in [PolicyFormat::Json, PolicyFormat::Yaml] {
        for value in &invalid {
            assert!(
                parse(value, format).is_err(),
                "accepted {format:?}: {value}"
            );
        }
        for version in [0, 2, u64::MAX] {
            let mut value = valid.clone();
            value["version"] = json!(version);
            assert_eq!(
                parse(&value, format),
                Err(PolicyLoadError::UnsupportedVersion(version))
            );
        }
        let mut duplicated = valid.clone();
        duplicated["policies"]
            .as_array_mut()
            .unwrap()
            .push(valid["policies"][0].clone());
        assert_eq!(
            parse(&duplicated, format),
            Err(PolicyLoadError::InvalidPolicy(PolicyError::DuplicatePolicy))
        );
        let mut duplicated = valid.clone();
        duplicated["policies"][0]["rules"]
            .as_array_mut()
            .unwrap()
            .push(valid["policies"][0]["rules"][0].clone());
        assert_eq!(
            parse(&duplicated, format),
            Err(PolicyLoadError::InvalidPolicy(PolicyError::DuplicateRule))
        );
    }
}

#[test]
fn malformed_duplicate_and_multiple_documents_are_rejected() {
    for input in [
        "",
        "null",
        "[]",
        "{}",
        "{",
        "{\"version\":1,\"policies\":[]} trailing",
        "{\"version\":1,\"policies\":[]} {}",
        "{\"version\":1,\"version\":1,\"policies\":[]}",
        "{\"version\":1,\"policies\":[{\"id\":\"base\",\"id\":\"other\",\"level\":\"company\",\"rules\":[]}]}",
    ] {
        assert!(
            PolicyDocument::from_json(input.as_bytes()).is_err(),
            "accepted {input}"
        );
    }
    for input in [
        "",
        "null",
        "[]",
        "{}",
        "version: 1\nversion: 1\npolicies: []",
        "version: 1\npolicies: []\n---\nversion: 1\npolicies: []",
        "version: 1\npolicies: []\n---",
        "version: !custom 1\npolicies: []",
        "version: .nan\npolicies: []",
        "version: 1\npolicies: []\n1: value",
        "version: 1\npolicies: []\n<<: {version: 1}",
    ] {
        assert!(
            PolicyDocument::from_yaml(input.as_bytes()).is_err(),
            "accepted {input}"
        );
    }
    for format in [PolicyFormat::Json, PolicyFormat::Yaml] {
        assert!(load_policy_document(&[0xff, 0xfe], format).is_err());
        let oversized = vec![b' '; MAX_POLICY_DOCUMENT_BYTES + 1];
        assert_eq!(
            load_policy_document(&oversized, format),
            Err(PolicyLoadError::TooLarge {
                actual: oversized.len(),
                maximum: MAX_POLICY_DOCUMENT_BYTES
            })
        );
        let mut boundary = b"{\"version\":1,\"policies\":[]}".to_vec();
        boundary.resize(MAX_POLICY_DOCUMENT_BYTES, b' ');
        assert!(load_policy_document(&boundary, format).is_ok());
    }
}

#[test]
fn structural_and_matcher_complexity_is_bounded() {
    let mut matcher = json!({"kind": "any"});
    for _ in 0..MAXIMUM_DEPTH {
        matcher = json!({"kind": "all", "matchers": [matcher]});
    }
    for format in [PolicyFormat::Json, PolicyFormat::Yaml] {
        let mut value = document(rule(json!({"kind": "allow"})));
        value["policies"][0]["rules"][0]["arguments"] = matcher.clone();
        assert!(parse(&value, format).is_ok());
        value["policies"][0]["rules"][0]["arguments"] =
            json!({"kind": "all", "matchers": [matcher.clone()]});
        assert!(parse(&value, format).is_err());
        let mut deep = json!(null);
        for _ in 0..MAX_POLICY_DOCUMENT_DEPTH + 1 {
            deep = json!([deep]);
        }
        assert!(parse(&deep, format).is_err());
        let broad = json!(vec![0; MAX_POLICY_DOCUMENT_NODES + 1]);
        assert!(
            matches!(parse(&broad, format), Err(PolicyLoadError::InvalidDocument(message)) if message.contains("complexity limit"))
        );
    }
    let recursive = "version: 1\npolicies: &loop [*loop]";
    assert!(PolicyDocument::from_yaml(recursive.as_bytes()).is_err());
    let expanded = format!(
        "version: 1\npolicies: [&text {}, {}]\n",
        "a".repeat(65_536),
        vec!["*text"; 20].join(", ")
    );
    assert!(expanded.len() < MAX_POLICY_DOCUMENT_BYTES);
    assert!(matches!(
        PolicyDocument::from_yaml(expanded.as_bytes()),
        Err(PolicyLoadError::InvalidDocument(message)) if message.contains("expanded text limit")
    ));
}

#[test]
fn hierarchy_is_stable_and_lower_levels_cannot_override_denial() {
    let mut policies = Vec::new();
    for (id, level, effect) in [
        ("session", "session", "allow"),
        ("agent", "agent", "allow"),
        ("team", "team", "allow"),
        ("department", "department", "ask"),
        ("base", "company", "deny"),
        ("another", "company", "log"),
    ] {
        policies.push(json!({"id": id, "level": level, "rules": [rule(json!({"kind": effect}))]}));
    }
    let (session, action) = request();
    let mut baseline = None;
    for format in [PolicyFormat::Json, PolicyFormat::Yaml] {
        for _ in 0..policies.len() {
            policies.rotate_left(1);
            let value = json!({"version": 1, "policies": policies});
            let loaded = parse(&value, format).unwrap();
            assert_eq!(
                loaded
                    .policies()
                    .iter()
                    .map(|policy| policy.level)
                    .collect::<Vec<_>>(),
                vec![
                    PolicyLevel::Company,
                    PolicyLevel::Company,
                    PolicyLevel::Department,
                    PolicyLevel::Team,
                    PolicyLevel::Agent,
                    PolicyLevel::Session
                ]
            );
            let decision = loaded.into_engine().evaluate(&session, &action, 20);
            assert_eq!(decision.outcome, Outcome::Deny);
            assert_eq!(decision.reason, Reason::ExplicitDeny);
            assert_eq!(decision.matches[0].policy, "another");
            assert_eq!(baseline.get_or_insert_with(|| decision.clone()), &decision);
        }
    }
}

#[test]
fn dry_run_summarizes_every_outcome_reason_and_obligation() {
    let mut rules = Vec::new();
    for (name, effects) in [
        ("allow", vec![json!({"kind": "allow"})]),
        ("deny", vec![json!({"kind": "deny"})]),
        (
            "ask",
            vec![
                json!({"kind": "ask"}),
                json!({"kind": "limit", "budget": "reads", "maximum": 2}),
            ],
        ),
        (
            "limited",
            vec![
                json!({"kind": "allow"}),
                json!({"kind": "limit", "budget": "reads", "maximum": 5}),
                json!({"kind": "limit", "budget": "reads", "maximum": 3}),
            ],
        ),
        ("log", vec![json!({"kind": "log"})]),
        (
            "budget",
            vec![json!({"kind": "limit", "budget": "reads", "maximum": 0})],
        ),
    ] {
        for (index, effect) in effects.into_iter().enumerate() {
            let mut value = rule(effect);
            value["id"] = json!(format!("{name}-{index}"));
            value["action"] = json!({"kind": "exact", "value": format!("item.{name}")});
            rules.push(value);
        }
    }
    for index in 0..2 {
        let mut value = rule(json!({"kind": "log"}));
        value["id"] = json!(format!("log-all-{index}"));
        value["action"] = json!({"kind": "any"});
        rules.push(value);
    }
    let engine = parse(
        &json!({"version": 1, "policies": [{"id": "base", "level": "company", "rules": rules}]}),
        PolicyFormat::Yaml,
    )
    .unwrap()
    .into_engine();
    let (session, action) = request();
    let missing = Session::new(
        "test-session".into(),
        session.identity().clone(),
        vec!["absent".into()],
        10,
    )
    .unwrap();
    let actions = [
        "allow",
        "deny",
        "ask",
        "limited",
        "log",
        "budget",
        "unmatched",
    ]
    .map(|name| {
        let mut action = action.clone();
        action.name = format!("item.{name}");
        action
    });
    let mut inputs: Vec<_> = actions
        .iter()
        .map(|action| EvaluationInput {
            session: &session,
            action,
            now: 20,
        })
        .collect();
    inputs.push(EvaluationInput {
        session: &session,
        action: &action,
        now: 100,
    });
    inputs.push(EvaluationInput {
        session: &missing,
        action: &action,
        now: 20,
    });
    let report = dry_run(&engine, &inputs);
    let summary = &report.summary;
    assert_eq!(summary.total, 9);
    assert_eq!(
        (
            summary.allowed,
            summary.denied,
            summary.approval_required,
            summary.limited
        ),
        (1, 6, 1, 1)
    );
    assert_eq!(
        (
            summary.explicit_denials,
            summary.default_denials,
            summary.invalid_actions,
            summary.missing_policies
        ),
        (1, 3, 1, 1)
    );
    assert_eq!(summary.logged_actions, 7);
    assert_eq!(
        summary.budgets["reads"],
        BudgetSummary {
            actions: 3,
            strictest_maximum: 0
        }
    );
    assert_eq!(
        summary.rule_matches[&MatchedRule {
            policy: "base".into(),
            rule: "log-all-0".into()
        }],
        7
    );
    assert_eq!(report.decisions[3].limits["reads"], 3);
    assert_eq!(report, dry_run(&engine, &inputs));
    inputs.reverse();
    assert_eq!(*summary, dry_run(&engine, &inputs).summary);
    assert_eq!(*summary, summarize_decisions(&report.decisions));
    assert_eq!(dry_run(&engine, &[]).summary, DryRunSummary::default());
    assert!(dry_run(&engine, &[]).decisions.is_empty());
    assert_eq!(
        session.lifecycle().state(),
        latch_core::SessionState::Active
    );
}

#[test]
fn scenarios_check_all_expectations_and_validate_ids() {
    let engine = parse(
        &document(rule(json!({"kind": "allow"}))),
        PolicyFormat::Json,
    )
    .unwrap()
    .into_engine();
    let (session, action) = request();
    let input = EvaluationInput {
        session: &session,
        action: &action,
        now: 20,
    };
    let expected = DecisionExpectation {
        outcome: Outcome::Allow,
        reason: Some(Reason::ExplicitAllow),
        matched_rules: Some(vec![MatchedRule {
            policy: "base".into(),
            rule: "read".into(),
        }]),
        limits: Some(Default::default()),
    };
    let good = PolicyScenario {
        id: "good".into(),
        input,
        expected,
    };
    let mut bad = good.clone();
    bad.id = "bad".into();
    bad.expected = DecisionExpectation {
        outcome: Outcome::Limited,
        reason: Some(Reason::BudgetRequired),
        matched_rules: Some(Vec::new()),
        limits: Some([("reads".into(), 1)].into()),
    };
    let report = test_scenarios(&engine, &[good.clone(), bad]).unwrap();
    assert_eq!((report.passed, report.failed), (1, 1));
    assert!(report.results[0].passed());
    let mismatches = &report.results[1].mismatches;
    assert_eq!(mismatches.len(), 4);
    assert!(matches!(&mismatches[0], ScenarioMismatch::Outcome { .. }));
    assert!(matches!(
        &mismatches[1],
        ScenarioMismatch::Reason {
            expected: Reason::BudgetRequired,
            actual: Reason::ExplicitAllow
        }
    ));
    assert!(
        matches!(&mismatches[2], ScenarioMismatch::MatchedRules { expected, actual } if expected.is_empty() && actual.len() == 1)
    );
    assert!(
        matches!(&mismatches[3], ScenarioMismatch::Limits { expected, actual } if expected["reads"] == 1 && actual.is_empty())
    );
    assert_eq!(
        test_scenarios(&engine, &[good.clone(), good.clone()]),
        Err(ScenarioError::DuplicateId("good".into()))
    );
    for id in ["", " ", "bad\nid"] {
        let mut invalid = good.clone();
        invalid.id = id.into();
        assert_eq!(
            test_scenarios(&engine, &[invalid]),
            Err(ScenarioError::InvalidId)
        );
    }
    assert_eq!(
        test_scenarios(&engine, &[]).unwrap(),
        ScenarioReport {
            passed: 0,
            failed: 0,
            results: vec![]
        }
    );
}

#[test]
fn empty_policies_fail_closed_and_selectors_are_literal() {
    let (session, mut action) = request();
    for format in [PolicyFormat::Json, PolicyFormat::Yaml] {
        let engine = parse(&json!({"version": 1, "policies": []}), format)
            .unwrap()
            .into_engine();
        assert_eq!(
            engine.evaluate(&session, &action, 20).reason,
            Reason::MissingPolicy("base".into())
        );
        let engine = parse(
            &json!({"version": 1, "policies": [{"id": "base", "level": "company", "rules": []}]}),
            format,
        )
        .unwrap()
        .into_engine();
        assert_eq!(
            engine.evaluate(&session, &action, 20).reason,
            Reason::DefaultDeny
        );
        let mut value = rule(json!({"kind": "allow"}));
        value["resource"] = json!({"kind": "exact", "value": "items/*"});
        let engine = parse(&document(value), format).unwrap().into_engine();
        action.resource = "items/example".into();
        assert_eq!(
            engine.evaluate(&session, &action, 20).outcome,
            Outcome::Deny
        );
        action.resource = "items/*".into();
        assert_eq!(
            engine.evaluate(&session, &action, 20).outcome,
            Outcome::Allow
        );
    }
}

#[test]
fn dry_run_and_scenarios_are_deterministic_and_do_not_execute() {
    let engine = parse(
        &document(rule(json!({"kind": "allow"}))),
        PolicyFormat::Json,
    )
    .unwrap()
    .into_engine();
    let (session, action) = request();
    let mut unmatched = action.clone();
    unmatched.name = "item.inspect".into();
    let inputs = [
        EvaluationInput {
            session: &session,
            action: &action,
            now: 20,
        },
        EvaluationInput {
            session: &session,
            action: &unmatched,
            now: 20,
        },
    ];
    let report = dry_run(&engine, &inputs);
    assert_eq!(report, dry_run(&engine, &inputs));
    assert_eq!(report.summary.total, 2);
    assert_eq!(report.summary.allowed, 1);
    assert_eq!(report.summary.denied, 1);
    assert_eq!(report.decisions[1].reason, Reason::DefaultDeny);
    let scenarios = [
        PolicyScenario {
            id: "allowed".into(),
            input: inputs[0],
            expected: DecisionExpectation::outcome(Outcome::Allow),
        },
        PolicyScenario {
            id: "mismatch".into(),
            input: inputs[1],
            expected: DecisionExpectation::outcome(Outcome::Allow),
        },
    ];
    let results = test_scenarios(&engine, &scenarios).unwrap();
    assert_eq!(results, test_scenarios(&engine, &scenarios).unwrap());
    assert_eq!(results.passed, 1);
    assert_eq!(results.failed, 1);
    assert_eq!(
        results.results[1].mismatches,
        vec![ScenarioMismatch::Outcome {
            expected: Outcome::Allow,
            actual: Outcome::Deny
        }]
    );
}
