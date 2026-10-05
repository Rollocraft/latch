use crate::TransitionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    MissingField,
    InvalidIdentity,
    InvalidTrust,
    ExpiredIdentity,
    AttributionMismatch,
    InactiveSession,
    InvalidAction,
    Transition(TransitionError),
}
