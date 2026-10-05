use super::finance_test_support::*;
use super::*;

#[test]
fn invalid_transitions_and_unknown_ids_are_atomic() {
    let (mut state, scope) = configured();
    let before = snapshot(&state);
    assert_eq!(
        state.begin(&scope, "missing"),
        Err(BudgetError::UnknownReservation)
    );
    assert_eq!(
        state.cancel(&scope, "missing"),
        Err(BudgetError::UnknownReservation)
    );
    assert_eq!(
        state.commit(&scope, "missing", OperationOutcome::Ambiguous),
        Err(BudgetError::UnknownReservation)
    );
    assert_eq!(
        state.reserve(
            &super::finance_test_support::scope("missing", "s"),
            "r",
            &amounts(1, 1)
        ),
        Err(BudgetError::UnknownScope)
    );
    assert_eq!(state, before);
    state.reserve(&scope, "r", &amounts(1, 1)).unwrap();
    let before = snapshot(&state);
    assert_eq!(
        state.commit(&scope, "r", OperationOutcome::Applied),
        Err(BudgetError::InvalidTransition)
    );
    assert_eq!(state, before);
    state.begin(&scope, "r").unwrap();
    let before = snapshot(&state);
    assert_eq!(
        state.begin(&scope, "r"),
        Err(BudgetError::InvalidTransition)
    );
    assert_eq!(state, before);
    assert_eq!(
        state
            .balance(&scope, UsageDimension::ApiTokens)
            .unwrap()
            .reserved(),
        1
    );
}
