use super::finance_dimension_validation::collect_dimensions;
use super::finance_reservation::Reservation;
use super::finance_scope::validate_identifier;
use super::{
    BudgetError, BudgetScope, BudgetState, MAX_RESERVATIONS, ReservationState, UsageAmount,
    UsageDimension,
};

impl BudgetState {
    pub fn reserve(
        &mut self,
        scope: &BudgetScope,
        id: &str,
        amounts: &[(UsageDimension, UsageAmount)],
    ) -> Result<ReservationState, BudgetError> {
        validate_identifier(id)?;
        let key = (scope.clone(), id.into());
        if self.reservations.contains_key(&key) {
            return Err(BudgetError::DuplicateReservation);
        }
        if self.reservations.len() >= MAX_RESERVATIONS {
            return Err(BudgetError::ReservationCapacityReached);
        }
        let mut balances = self
            .scopes
            .get(scope)
            .ok_or(BudgetError::UnknownScope)?
            .clone();
        let amounts = collect_dimensions(amounts)?;
        for (&dimension, &amount) in &amounts {
            balances
                .get_mut(&dimension)
                .ok_or(BudgetError::UnknownDimension(dimension))?
                .reserve(dimension, amount)?;
        }
        self.scopes.insert(scope.clone(), balances);
        self.reservations.insert(
            key,
            Reservation {
                amounts,
                state: ReservationState::Reserved,
            },
        );
        Ok(ReservationState::Reserved)
    }
}
