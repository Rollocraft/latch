use super::UsageDimension;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetError {
    InvalidIdentifier,
    InvalidDimensionCount,
    ScopeCapacityReached,
    ReservationCapacityReached,
    ScopeAlreadyConfigured,
    UnknownScope,
    DuplicateDimension(UsageDimension),
    UnknownDimension(UsageDimension),
    UnitMismatch(UsageDimension),
    CurrencyMismatch(UsageDimension),
    LimitExceeded(UsageDimension),
    ArithmeticOverflow,
    DuplicateReservation,
    UnknownReservation,
    InvalidTransition,
}
