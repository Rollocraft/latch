use super::ExecutionError;
use latch_core::Action;
use latch_policy::Decision;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRecord {
    pub id: u64,
    pub action: Action,
    pub organization: String,
    pub session: String,
    pub owner: String,
    pub issuer: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub revoked: bool,
    pub consumed: bool,
}

#[derive(Debug)]
pub enum CapabilityError<S = std::convert::Infallible> {
    InvalidIssuer,
    InvalidExpiration,
    Unknown,
    ScopeMismatch,
    NotYetValid,
    Expired,
    Revoked,
    Consumed,
    AlreadyAttempted,
    IdentifierExhausted,
    Random(getrandom::Error),
    TokenCollision,
    Policy(Box<Decision>),
    Audit(S),
}

#[derive(Debug)]
pub enum CapabilityExecutionError<E, S = std::convert::Infallible> {
    Capability(CapabilityError<S>),
    Execution(ExecutionError<E, S>),
}
