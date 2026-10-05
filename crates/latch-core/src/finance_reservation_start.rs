use super::finance_scope::validate_identifier;
use super::{BudgetError, BudgetScope, BudgetState, ReservationState};

impl BudgetState {
    pub fn reservation_state(
        &self,
        scope: &BudgetScope,
        id: &str,
    ) -> Result<ReservationState, BudgetError> {
        validate_identifier(id)?;
        self.reservations
            .get(&(scope.clone(), id.into()))
            .map(|reservation| reservation.state)
            .ok_or(BudgetError::UnknownReservation)
    }

    pub fn begin(
        &mut self,
        scope: &BudgetScope,
        id: &str,
    ) -> Result<ReservationState, BudgetError> {
        validate_identifier(id)?;
        let reservation = self
            .reservations
            .get_mut(&(scope.clone(), id.into()))
            .ok_or(BudgetError::UnknownReservation)?;
        if reservation.state != ReservationState::Reserved {
            return Err(BudgetError::InvalidTransition);
        }
        reservation.state = ReservationState::InFlight;
        Ok(reservation.state)
    }
}
