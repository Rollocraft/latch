use crate::{Decision, MatchedRule, Outcome, Reason};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioMismatch {
    Outcome {
        expected: Outcome,
        actual: Outcome,
    },
    Reason {
        expected: Reason,
        actual: Reason,
    },
    MatchedRules {
        expected: Vec<MatchedRule>,
        actual: Vec<MatchedRule>,
    },
    Limits {
        expected: BTreeMap<String, u64>,
        actual: BTreeMap<String, u64>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioResult {
    pub id: String,
    pub decision: Decision,
    pub mismatches: Vec<ScenarioMismatch>,
}

impl ScenarioResult {
    pub fn passed(&self) -> bool {
        self.mismatches.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReport {
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<ScenarioResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioError {
    InvalidId,
    DuplicateId(String),
}

impl fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId => f.write_str("invalid scenario ID"),
            Self::DuplicateId(id) => write!(f, "duplicate scenario ID: {id}"),
        }
    }
}

impl std::error::Error for ScenarioError {}
