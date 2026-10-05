use super::{BudgetError, BudgetScope, BudgetState, OperationOutcome, ReservationState};

impl BudgetState {
    pub fn commit(
        &mut self,
        scope: &BudgetScope,
        id: &str,
        outcome: OperationOutcome,
    ) -> Result<ReservationState, BudgetError> {
        if self.reservation_state(scope, id)? != ReservationState::InFlight {
            return Err(BudgetError::InvalidTransition);
        }
        let state = match outcome {
            OperationOutcome::Applied => ReservationState::Committed,
            OperationOutcome::Ambiguous => ReservationState::ConsumedAmbiguous,
        };
        self.finish(scope, id, state)
    }

    pub fn cancel(
        &mut self,
        scope: &BudgetScope,
        id: &str,
    ) -> Result<ReservationState, BudgetError> {
        let state = match self.reservation_state(scope, id)? {
            ReservationState::Reserved => ReservationState::CancelledBeforeExecution,
            ReservationState::InFlight => ReservationState::ConsumedAmbiguous,
            _ => return Err(BudgetError::InvalidTransition),
        };
        self.finish(scope, id, state)
    }
}
