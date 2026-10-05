use super::finance_test_support::*;
use super::*;

#[test]
fn multiple_reservations_commit_and_cancel_without_interference() {
    let (mut state, scope) = configured();
    state.reserve(&scope, "a", &amounts(40, 20_000)).unwrap();
    state.reserve(&scope, "b", &amounts(60, 30_000)).unwrap();
    state.begin(&scope, "a").unwrap();
    state
        .commit(&scope, "a", OperationOutcome::Applied)
        .unwrap();
    state.cancel(&scope, "b").unwrap();
    let balance = state.balance(&scope, UsageDimension::ApiTokens).unwrap();
    assert_eq!(balance.reserved(), 0);
    assert_eq!(balance.consumed(), 40);
    assert_eq!(balance.available(), Ok(60));
    state.reserve(&scope, "c", &amounts(60, 30_000)).unwrap();
}

#[test]
fn zero_limits_and_invalid_configuration_fail_closed() {
    let mut state = BudgetState::new();
    let scope = scope("t", "s");
    for limits in [
        vec![],
        vec![(UsageDimension::Refunds, UsageAmount::Units(0))],
        vec![(UsageDimension::ApiTokens, UsageAmount::Units(0)); 2],
    ] {
        let before = snapshot(&state);
        assert!(state.configure_scope(scope.clone(), &limits).is_err());
        assert_eq!(state, before);
    }
    state
        .configure_scope(scope.clone(), &amounts(0, 0))
        .unwrap();
    state.reserve(&scope, "zero", &amounts(0, 0)).unwrap();
    state.cancel(&scope, "zero").unwrap();
    let before = snapshot(&state);
    assert_eq!(
        state.cancel(&scope, "zero"),
        Err(BudgetError::InvalidTransition)
    );
    assert_eq!(
        state.begin(&scope, "zero"),
        Err(BudgetError::InvalidTransition)
    );
    assert_eq!(
        state.commit(&scope, "zero", OperationOutcome::Ambiguous),
        Err(BudgetError::InvalidTransition)
    );
    assert_eq!(
        state.reserve(&scope, "positive", &amounts(0, 1)),
        Err(BudgetError::LimitExceeded(UsageDimension::Refunds))
    );
    assert_eq!(state, before);
}
