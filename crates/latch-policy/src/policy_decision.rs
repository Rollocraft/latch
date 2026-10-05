use crate::{Effect, PolicyLevel};
use latch_core::ValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Allow,
    Deny,
    ApprovalRequired,
    Limited,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleMatch {
    pub policy: String,
    pub level: PolicyLevel,
    pub rule: String,
    pub description: String,
    pub effect: Effect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    InvalidAction(ValidationError),
    MissingPolicy(String),
    ExplicitDeny,
    DefaultDeny,
    ApprovalRequired,
    BudgetRequired,
    ExplicitAllow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub outcome: Outcome,
    pub reason: Reason,
    pub matches: Vec<RuleMatch>,
    /// All matching budgets survive hierarchy merging, including when approval
    /// is also needed. Repeated budget names use the lowest maximum.
    pub limits: std::collections::BTreeMap<String, u64>,
}
