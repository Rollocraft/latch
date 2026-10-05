use crate::{Decision, MatchedRule};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DryRunSummary {
    pub total: usize,
    pub allowed: usize,
    pub denied: usize,
    pub approval_required: usize,
    pub limited: usize,
    pub explicit_denials: usize,
    pub default_denials: usize,
    pub invalid_actions: usize,
    pub missing_policies: usize,
    pub logged_actions: usize,
    pub rule_matches: BTreeMap<MatchedRule, usize>,
    pub budgets: BTreeMap<String, BudgetSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetSummary {
    pub actions: usize,
    pub strictest_maximum: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DryRunReport {
    pub summary: DryRunSummary,
    pub decisions: Vec<Decision>,
}
