use super::{BudgetError, BudgetLedger, BudgetScope};
use std::collections::BTreeMap;

fn validate_costs(
    limits: &BTreeMap<String, u64>,
    costs: &BTreeMap<String, u64>,
) -> Result<(), BudgetError> {
    for name in limits.keys() {
        if name.trim().is_empty() || name.chars().any(char::is_control) {
            return Err(BudgetError::InvalidBudget);
        }
        if !costs.contains_key(name) {
            return Err(BudgetError::MissingCost(name.clone()));
        }
    }
    for name in costs.keys() {
        if !limits.contains_key(name) {
            return Err(BudgetError::UnexpectedCost(name.clone()));
        }
    }
    Ok(())
}

impl BudgetLedger {
    pub fn charge(
        &self,
        scope: &BudgetScope,
        limits: &BTreeMap<String, u64>,
        costs: &BTreeMap<String, u64>,
    ) -> Result<(), BudgetError> {
        validate_costs(limits, costs)?;
        let mut usage = self.usage.lock().map_err(|_| BudgetError::Unavailable)?;
        let mut next = usage.get(scope).cloned().unwrap_or_default();
        for (name, maximum) in limits {
            let total = next
                .get(name)
                .copied()
                .unwrap_or(0)
                .checked_add(costs[name])
                .ok_or_else(|| BudgetError::Exhausted(name.clone()))?;
            if total > *maximum {
                return Err(BudgetError::Exhausted(name.clone()));
            }
            next.insert(name.clone(), total);
        }
        if !limits.is_empty() {
            usage.insert(scope.clone(), next);
        }
        Ok(())
    }
}
