use super::finance_dimension_validation::collect_dimensions;
use super::{
    BudgetBalance, BudgetError, BudgetScope, BudgetState, MAX_BUDGET_SCOPES, UsageAmount,
    UsageDimension,
};

impl BudgetState {
    pub fn configure_scope(
        &mut self,
        scope: BudgetScope,
        limits: &[(UsageDimension, UsageAmount)],
    ) -> Result<(), BudgetError> {
        if self.scopes.contains_key(&scope) {
            return Err(BudgetError::ScopeAlreadyConfigured);
        }
        if self.scopes.len() >= MAX_BUDGET_SCOPES {
            return Err(BudgetError::ScopeCapacityReached);
        }
        let balances = collect_dimensions(limits)?
            .into_iter()
            .map(|(dimension, limit)| {
                (
                    dimension,
                    BudgetBalance {
                        limit,
                        reserved: 0,
                        consumed: 0,
                    },
                )
            })
            .collect();
        self.scopes.insert(scope, balances);
        Ok(())
    }

    pub fn balance(
        &self,
        scope: &BudgetScope,
        dimension: UsageDimension,
    ) -> Result<BudgetBalance, BudgetError> {
        self.scopes
            .get(scope)
            .ok_or(BudgetError::UnknownScope)?
            .get(&dimension)
            .copied()
            .ok_or(BudgetError::UnknownDimension(dimension))
    }
}
