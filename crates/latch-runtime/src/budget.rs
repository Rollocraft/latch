//! Atomic accounting for named session budgets. Costs must be measured or bounded
//! by the trusted adapter, never accepted as an agent's self-reported usage.

use std::{collections::BTreeMap, sync::Mutex};

#[path = "budget_charge.rs"]
mod budget_charge;
#[path = "budget_scope.rs"]
mod budget_scope;
pub use budget_scope::{BudgetError, BudgetScope};

type Usage = BTreeMap<BudgetScope, BTreeMap<String, u64>>;

/// Share one ledger across workers; creating a new ledger resets accounting.
/// Charges are deliberately not refunded on execution failure: effects may have
/// occurred before the failure became observable. All counters change together.
#[derive(Debug, Default)]
pub struct BudgetLedger {
    usage: Mutex<Usage>,
}

impl BudgetLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn usage(&self, scope: &BudgetScope) -> Result<BTreeMap<String, u64>, BudgetError> {
        Ok(self
            .usage
            .lock()
            .map_err(|_| BudgetError::Unavailable)?
            .get(scope)
            .cloned()
            .unwrap_or_default())
    }
}

#[cfg(test)]
#[path = "budget_accounting_tests.rs"]
mod budget_accounting_tests;
