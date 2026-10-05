use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Denied,
    ApprovalRequired,
    Allowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionReason {
    PolicyAllowed,
    PolicyDenied,
    NoMatchingPolicy,
    ApprovalRequired,
    IdentityMismatch,
    InactiveSession,
    ResourceRestricted,
    EnvironmentRestricted,
    ArgumentRestricted,
    RiskLimitExceeded,
    BudgetExceeded,
    InvalidRequest,
    UnsupportedVersion,
    RateLimited,
    IdempotencyConflict,
    InternalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidRequest,
    UnsupportedVersion,
    Unauthenticated,
    Forbidden,
    NotFound,
    RateLimited,
    IdempotencyConflict,
    Internal,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Outcome {
    Decision {
        decision: Decision,
        reasons: Vec<DecisionReason>,
    },
    Error {
        code: ErrorCode,
        reasons: Vec<DecisionReason>,
    },
}
