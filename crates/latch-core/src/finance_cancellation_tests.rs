use super::finance_test_support::*;
use super::*;

#[test]
fn only_cancellation_before_execution_releases_budget() {
    let (mut state, scope) = configured();
    state.reserve(&scope, "safe", &amounts(20, 5_000)).unwrap();
    assert_eq!(
        state.cancel(&scope, "safe"),
        Ok(ReservationState::CancelledBeforeExecution)
    );
    assert_eq!(
        state
            .balance(&scope, UsageDimension::Refunds)
            .unwrap()
            .available(),
        Ok(50_000)
    );
    for (id, outcome) in [
        ("applied", Some(OperationOutcome::Applied)),
        ("unknown", Some(OperationOutcome::Ambiguous)),
        ("cancelled-in-flight", None),
    ] {
        state.reserve(&scope, id, &amounts(20, 5_000)).unwrap();
        state.begin(&scope, id).unwrap();
        let expected = if outcome == Some(OperationOutcome::Applied) {
            ReservationState::Committed
        } else {
            ReservationState::ConsumedAmbiguous
        };
        let result = match outcome {
            Some(outcome) => state.commit(&scope, id, outcome),
            None => state.cancel(&scope, id),
        };
        assert_eq!(result, Ok(expected));
        let before = snapshot(&state);
        assert_eq!(
            state.cancel(&scope, id),
            Err(BudgetError::InvalidTransition)
        );
        assert_eq!(
            state.commit(&scope, id, OperationOutcome::Applied),
            Err(BudgetError::InvalidTransition)
        );
        assert_eq!(state.begin(&scope, id), Err(BudgetError::InvalidTransition));
        assert_eq!(state, before);
    }
    let balance = state.balance(&scope, UsageDimension::Refunds).unwrap();
    assert_eq!(balance.reserved(), 0);
    assert_eq!(balance.consumed(), 15_000);
    assert_eq!(balance.available(), Ok(35_000));
}
