use super::{UsageAmount, UsageDimension};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationState {
    Reserved,
    InFlight,
    Committed,
    CancelledBeforeExecution,
    ConsumedAmbiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationOutcome {
    Applied,
    Ambiguous,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Reservation {
    pub(super) amounts: BTreeMap<UsageDimension, UsageAmount>,
    pub(super) state: ReservationState,
}
