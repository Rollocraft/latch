use super::{BudgetError, BudgetScope, BudgetState, ReservationState};

impl BudgetState {
    pub(super) fn finish(
        &mut self,
        scope: &BudgetScope,
        id: &str,
        state: ReservationState,
    ) -> Result<ReservationState, BudgetError> {
        let key = (scope.clone(), id.into());
        let mut reservation = self
            .reservations
            .get(&key)
            .ok_or(BudgetError::UnknownReservation)?
            .clone();
        let mut balances = self
            .scopes
            .get(scope)
            .ok_or(BudgetError::UnknownScope)?
            .clone();
        for (&dimension, &amount) in &reservation.amounts {
            balances
                .get_mut(&dimension)
                .ok_or(BudgetError::UnknownDimension(dimension))?
                .finish(amount, state)?;
        }
        reservation.state = state;
        self.scopes.insert(scope.clone(), balances);
        self.reservations.insert(key, reservation);
        Ok(state)
    }
}
