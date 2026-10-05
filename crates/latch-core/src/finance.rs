//! Currency-bound money, approval thresholds, and atomic budget reservations.

pub const MAX_SCOPE_BYTES: usize = 128;
pub const MAX_BUDGET_SCOPES: usize = 1_024;
pub const MAX_RESERVATIONS: usize = 4_096;
pub const MAX_USAGE_DIMENSIONS: usize = 7;

#[path = "finance_balance.rs"]
mod finance_balance;
#[path = "finance_dimension_validation.rs"]
mod finance_dimension_validation;
#[path = "finance_errors.rs"]
mod finance_errors;
#[path = "finance_ledger.rs"]
mod finance_ledger;
#[path = "finance_money.rs"]
mod finance_money;
#[path = "finance_policy.rs"]
mod finance_policy;
#[path = "finance_reservation.rs"]
mod finance_reservation;
#[path = "finance_reservation_allocate.rs"]
mod finance_reservation_allocate;
#[path = "finance_reservation_finish.rs"]
mod finance_reservation_finish;
#[path = "finance_reservation_outcome.rs"]
mod finance_reservation_outcome;
#[path = "finance_reservation_start.rs"]
mod finance_reservation_start;
#[path = "finance_scope.rs"]
mod finance_scope;
#[path = "finance_scope_configuration.rs"]
mod finance_scope_configuration;
#[path = "finance_usage.rs"]
mod finance_usage;

pub use finance_balance::BudgetBalance;
pub use finance_errors::BudgetError;
pub use finance_ledger::BudgetState;
pub use finance_money::{Currency, FinanceError, Money};
pub use finance_policy::{
    FinancialDecision, FinancialPermission, FinancialPolicy, FinancialReason,
};
pub use finance_reservation::{OperationOutcome, ReservationState};
pub use finance_scope::BudgetScope;
pub use finance_usage::{UsageAmount, UsageDimension};

#[cfg(test)]
#[path = "finance_atomic_reserve_tests.rs"]
mod finance_atomic_reserve_tests;
#[cfg(test)]
#[path = "finance_balances_tests.rs"]
mod finance_balances_tests;
#[cfg(test)]
#[path = "finance_cancellation_tests.rs"]
mod finance_cancellation_tests;
#[cfg(test)]
#[path = "finance_capacity_tests.rs"]
mod finance_capacity_tests;
#[cfg(test)]
#[path = "finance_dimensions_tests.rs"]
mod finance_dimensions_tests;
#[cfg(test)]
#[path = "finance_duplicate_reservations_tests.rs"]
mod finance_duplicate_reservations_tests;
#[cfg(test)]
#[path = "finance_finalization_tests.rs"]
mod finance_finalization_tests;
#[cfg(test)]
#[path = "finance_invalid_requests_tests.rs"]
mod finance_invalid_requests_tests;
#[cfg(test)]
#[path = "finance_invalid_transitions_tests.rs"]
mod finance_invalid_transitions_tests;
#[cfg(test)]
#[path = "finance_money_tests.rs"]
mod finance_money_tests;
#[cfg(test)]
#[path = "finance_overflow_tests.rs"]
mod finance_overflow_tests;
#[cfg(test)]
#[path = "finance_policy_tests.rs"]
mod finance_policy_tests;
#[cfg(test)]
#[path = "finance_scope_isolation_tests.rs"]
mod finance_scope_isolation_tests;
#[cfg(test)]
#[path = "finance_test_support.rs"]
mod finance_test_support;
