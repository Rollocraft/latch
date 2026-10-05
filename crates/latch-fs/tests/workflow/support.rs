use latch_core::*;
use latch_fs::*;
use latch_policy::*;
use latch_runtime::gate::*;
use latch_transaction::{MAXIMUM_ENTRIES, Snapshot, Transaction};
use std::path::PathBuf;

pub struct Fixture {
    base: PathBuf,
}

impl Fixture {
    pub fn new(label: &str) -> Self {
        let base = std::env::temp_dir().join(format!(
            "latch-fs-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(base.join("root/src")).unwrap();
        std::fs::write(base.join("root/src/main.rs"), "fn main() {}").unwrap();
        std::fs::write(base.join("root/secret.env"), "TOKEN=live").unwrap();
        Self { base }
    }
    pub fn root(&self) -> PathBuf {
        self.base.join("root")
    }
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::of(self.root(), MAXIMUM_ENTRIES).unwrap()
    }
    pub fn read(&self, relative: &str) -> String {
        std::fs::read_to_string(self.root().join(relative)).unwrap()
    }
    pub fn adapter(&self) -> FileAdapter {
        FileAdapter::new(Transaction::begin(self.root(), self.base.join("workspace")).unwrap())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

pub fn identity() -> AgentIdentity {
    AgentIdentity {
        id: "agent://acme/backend/coder".into(),
        organization: "acme".into(),
        team: "backend".into(),
        owner: "david".into(),
        purpose: "refactoring".into(),
        provider: "local".into(),
        model: "coder".into(),
        model_version: "1".into(),
        runtime: "latch".into(),
        environment: "dev".into(),
        device: "workstation".into(),
        created_at: 10,
        expires_at: 1000,
        trust_level: 40,
    }
}

pub fn action(id: &str, name: &str, resource: &str, arguments: &[&str]) -> Action {
    Action {
        id: id.into(),
        actor: identity().id,
        session_id: "s1".into(),
        name: name.into(),
        resource: resource.into(),
        environment: "dev".into(),
        arguments: arguments.iter().map(|a| (*a).into()).collect(),
        reversibility: Reversibility::FullyReversible,
        risk: RiskScore::new(20).unwrap(),
    }
}

pub fn rule(id: &str, action: Selector, resource: Selector, effect: Effect) -> Rule {
    Rule {
        id: id.into(),
        description: format!("rule {id}"),
        action,
        resource,
        actor: Selector::Any,
        environment: Selector::Exact("dev".into()),
        arguments: ArgumentMatcher::Any,
        effect,
    }
}

/// Source files may be written within a budget, deletions need a human, and
/// the secret at the root of the tree is denied outright.
pub fn gate() -> ExecutionGate {
    let policies = PolicyEngine::new(vec![Policy {
        id: "default".into(),
        level: PolicyLevel::Company,
        rules: vec![
            rule(
                "deny-secrets",
                Selector::Any,
                Selector::Prefix("secret".into()),
                Effect::Deny,
            ),
            rule(
                "allow-source",
                Selector::Exact(WRITE.into()),
                Selector::Prefix("src/".into()),
                Effect::Allow,
            ),
            rule(
                "allow-moves",
                Selector::Exact(RENAME.into()),
                Selector::Prefix("src/".into()),
                Effect::Allow,
            ),
            rule(
                "review-deletions",
                Selector::Exact(DELETE.into()),
                Selector::Prefix("src/".into()),
                Effect::Ask,
            ),
            rule(
                "write-budget",
                Selector::Any,
                Selector::Prefix("src/".into()),
                Effect::Limit {
                    budget: BYTES.into(),
                    maximum: 64,
                },
            ),
        ],
    }])
    .unwrap();
    let session = Session::new("s1".into(), identity(), vec!["default".into()], 10).unwrap();
    ExecutionGate::new(session, policies)
}
