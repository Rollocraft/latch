use crate::ArgumentMatcher;
use latch_core::Action;

pub(crate) fn valid_text(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyLevel {
    Company,
    Department,
    Team,
    Agent,
    Session,
}

/// Exact matching is the default. Prefix matching is explicit and performs no
/// path normalization: filesystem adapters must supply canonical resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    Any,
    Exact(String),
    Prefix(String),
}

impl Selector {
    pub(crate) fn matches(&self, value: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(s) => s == value,
            Self::Prefix(s) => value.starts_with(s),
        }
    }
    pub(crate) fn valid(&self) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(s) | Self::Prefix(s) => valid_text(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Deny,
    Ask,
    /// A named budget must be atomically reserved by the runtime before execution.
    Limit {
        budget: String,
        maximum: u64,
    },
    /// Logging is additive and never grants permission by itself.
    Log,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub description: String,
    pub action: Selector,
    pub resource: Selector,
    pub environment: Selector,
    pub actor: Selector,
    /// What the invocation itself must look like. `Any` is the default and
    /// says the rule is about the action, not how it was called.
    pub arguments: ArgumentMatcher,
    pub effect: Effect,
}

impl Rule {
    pub(crate) fn matches(&self, action: &Action) -> bool {
        self.action.matches(&action.name)
            && self.resource.matches(&action.resource)
            && self.environment.matches(&action.environment)
            && self.actor.matches(&action.actor)
            && self.arguments.matches(&action.arguments)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub id: String,
    pub level: PolicyLevel,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    InvalidPolicy,
    DuplicatePolicy,
    DuplicateRule,
}
