use crate::{Outcome, Reason};
use latch_core::{Action, Session};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub struct EvaluationInput<'a> {
    pub session: &'a Session,
    pub action: &'a Action,
    pub now: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionExpectation {
    pub outcome: Outcome,
    pub reason: Option<Reason>,
    pub matched_rules: Option<Vec<MatchedRule>>,
    pub limits: Option<BTreeMap<String, u64>>,
}

impl DecisionExpectation {
    pub fn outcome(outcome: Outcome) -> Self {
        Self {
            outcome,
            reason: None,
            matched_rules: None,
            limits: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatchedRule {
    pub policy: String,
    pub rule: String,
}

#[derive(Debug, Clone)]
pub struct PolicyScenario<'a> {
    pub id: String,
    pub input: EvaluationInput<'a>,
    pub expected: DecisionExpectation,
}
