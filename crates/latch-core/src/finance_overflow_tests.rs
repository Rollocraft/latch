use super::finance_test_support::*;
use super::*;

#[test]
fn overflow_is_rejected_without_partial_updates() {
    let mut state = BudgetState::new();
    let scope = scope("t", "s");
    state
        .configure_scope(scope.clone(), &amounts(u64::MAX, u64::MAX))
        .unwrap();
    state.reserve(&scope, "max", &amounts(0, u64::MAX)).unwrap();
    let before = snapshot(&state);
    assert_eq!(
        state.reserve(&scope, "overflow", &amounts(1, 1)),
        Err(BudgetError::ArithmeticOverflow)
    );
    assert_eq!(state, before);
    state.begin(&scope, "max").unwrap();
    state
        .commit(&scope, "max", OperationOutcome::Ambiguous)
        .unwrap();
    let before = snapshot(&state);
    assert_eq!(
        state.reserve(&scope, "overflow", &amounts(1, 1)),
        Err(BudgetError::ArithmeticOverflow)
    );
    assert_eq!(state, before);
    state.reserve(&scope, "zero", &amounts(0, 0)).unwrap();
    state.cancel(&scope, "zero").unwrap();
    assert_eq!(
        state
            .balance(&scope, UsageDimension::Refunds)
            .unwrap()
            .consumed(),
        u64::MAX
    );
}
