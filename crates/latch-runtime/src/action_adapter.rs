use crate::budget::BudgetError;
use latch_core::Action;
use latch_policy::Decision;
use std::collections::BTreeMap;

pub trait ActionAdapter {
    type Output;
    type Error;
    /// Must be side-effect free and return conservative upper bounds, in the
    /// units of each named budget. Unknown budget units must return an error.
    fn costs(
        &self,
        action: &Action,
        limits: &BTreeMap<String, u64>,
    ) -> Result<BTreeMap<String, u64>, Self::Error>;
    /// Execute exactly this action and stay within the previously returned costs.
    fn execute(&mut self, action: &Action) -> Result<Self::Output, Self::Error>;
}

#[derive(Debug)]
pub enum ExecutionError<E, S = std::convert::Infallible> {
    Policy(Box<Decision>),
    Budget(BudgetError),
    Adapter(E),
    AlreadyAttempted,
    AuditBeforeExecution(S),
    /// Effects may already have occurred. The action ID remains consumed.
    AuditAfterExecution(S),
}
