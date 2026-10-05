use super::finance_test_support::*;
use super::*;

#[test]
fn duplicate_ids_never_reapply_or_replace_reservations() {
    for terminal in [
        ReservationState::Reserved,
        ReservationState::CancelledBeforeExecution,
        ReservationState::Committed,
        ReservationState::ConsumedAmbiguous,
    ] {
        let (mut state, scope) = configured();
        state.reserve(&scope, "r", &amounts(20, 5_000)).unwrap();
        match terminal {
            ReservationState::CancelledBeforeExecution => {
                state.cancel(&scope, "r").unwrap();
            }
            ReservationState::Committed | ReservationState::ConsumedAmbiguous => {
                state.begin(&scope, "r").unwrap();
                state
                    .commit(
                        &scope,
                        "r",
                        if terminal == ReservationState::Committed {
                            OperationOutcome::Applied
                        } else {
                            OperationOutcome::Ambiguous
                        },
                    )
                    .unwrap();
            }
            _ => {}
        }
        let before = snapshot(&state);
        for request in [amounts(20, 5_000), amounts(1, 1)] {
            assert_eq!(
                state.reserve(&scope, "r", &request),
                Err(BudgetError::DuplicateReservation)
            );
            assert_eq!(state, before);
        }
    }
}
