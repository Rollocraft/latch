use super::finance_test_support::*;
use super::*;

#[test]
fn checked_finalization_errors_leave_every_dimension_and_state_unchanged() {
    for overflow in [false, true] {
        let (mut state, scope) = configured();
        state.reserve(&scope, "r", &amounts(1, 1)).unwrap();
        state.begin(&scope, "r").unwrap();
        let balance = state
            .scopes
            .get_mut(&scope)
            .unwrap()
            .get_mut(&UsageDimension::Refunds)
            .unwrap();
        if overflow {
            balance.consumed = u64::MAX;
        } else {
            balance.reserved = 0;
        }
        let before = snapshot(&state);
        assert_eq!(
            state.commit(&scope, "r", OperationOutcome::Ambiguous),
            Err(BudgetError::ArithmeticOverflow)
        );
        assert_eq!(state, before);
        assert_eq!(
            state.cancel(&scope, "r"),
            Err(BudgetError::ArithmeticOverflow)
        );
        assert_eq!(state, before);
    }
}
