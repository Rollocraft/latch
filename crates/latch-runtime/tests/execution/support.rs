use latch_core::*;
use latch_policy::*;
use latch_runtime::gate::*;
use std::collections::BTreeMap;

pub fn setup(effect: Effect, maximum: u64) -> (ExecutionGate, Action) {
    setup_with_audit(effect, maximum, latch_audit::MemoryAudit::default())
}

pub fn setup_with_audit<S: latch_audit::AuditSink>(
    effect: Effect,
    maximum: u64,
    audit: S,
) -> (ExecutionGate<S>, Action) {
    let identity = AgentIdentity {
        id: "agent://acme/coder".into(),
        organization: "acme".into(),
        team: "dev".into(),
        owner: "david".into(),
        purpose: "coding".into(),
        provider: "local".into(),
        model: "coder".into(),
        model_version: "1".into(),
        runtime: "latch".into(),
        environment: "dev".into(),
        device: "pc".into(),
        created_at: 10,
        expires_at: 100,
        trust_level: 0,
    };
    let action = Action {
        id: "a1".into(),
        actor: identity.id.clone(),
        session_id: "s1".into(),
        name: "file.write".into(),
        resource: "project/readme".into(),
        environment: "dev".into(),
        arguments: vec![],
        reversibility: Reversibility::FullyReversible,
        risk: RiskScore::new(15).unwrap(),
    };
    let session = Session::new("s1".into(), identity, vec!["default".into()], 10).unwrap();
    let rule = |id: &str, effect| Rule {
        id: id.into(),
        description: id.into(),
        action: Selector::Any,
        resource: Selector::Any,
        actor: Selector::Any,
        environment: Selector::Any,
        arguments: latch_policy::ArgumentMatcher::Any,
        effect,
    };
    let engine = PolicyEngine::new(vec![Policy {
        id: "default".into(),
        level: PolicyLevel::Company,
        rules: vec![
            rule("decision", effect),
            rule(
                "budget",
                Effect::Limit {
                    budget: "writes".into(),
                    maximum,
                },
            ),
        ],
    }])
    .unwrap();
    (ExecutionGate::with_audit(session, engine, audit), action)
}

#[derive(Default)]
pub struct Adapter {
    pub estimates: std::cell::Cell<usize>,
    pub calls: usize,
    pub fail: bool,
}
impl ActionAdapter for Adapter {
    type Output = ();
    type Error = ();
    fn costs(&self, _: &Action, _: &BTreeMap<String, u64>) -> Result<BTreeMap<String, u64>, ()> {
        self.estimates.set(self.estimates.get() + 1);
        Ok(BTreeMap::from([("writes".into(), 1)]))
    }
    fn execute(&mut self, _: &Action) -> Result<(), ()> {
        self.calls += 1;
        if self.fail { Err(()) } else { Ok(()) }
    }
}

pub struct FailingAudit {
    pub remaining: usize,
}
impl latch_audit::AuditSink for FailingAudit {
    type Error = &'static str;
    fn append(&mut self, _: &latch_audit::AuditEvent) -> Result<(), Self::Error> {
        if self.remaining == 0 {
            return Err("disk full");
        }
        self.remaining -= 1;
        Ok(())
    }
}

#[derive(Default)]
pub struct RecoverableAudit {
    pub failed: std::cell::Cell<bool>,
    pub events: Vec<latch_audit::AuditEvent>,
}

impl latch_audit::AuditSink for RecoverableAudit {
    type Error = &'static str;

    fn append(&mut self, event: &latch_audit::AuditEvent) -> Result<(), Self::Error> {
        if self.failed.get() {
            Err("unavailable")
        } else {
            self.events.push(event.clone());
            Ok(())
        }
    }
}

pub struct PreparationFailure;
impl ActionAdapter for PreparationFailure {
    type Output = ();
    type Error = &'static str;
    fn costs(
        &self,
        _: &Action,
        _: &BTreeMap<String, u64>,
    ) -> Result<BTreeMap<String, u64>, Self::Error> {
        Err("unknown budget unit")
    }
    fn execute(&mut self, _: &Action) -> Result<(), Self::Error> {
        panic!("preparation failure must prevent execution")
    }
}
