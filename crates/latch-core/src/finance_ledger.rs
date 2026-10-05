use super::finance_reservation::Reservation;
use super::{BudgetBalance, BudgetScope, UsageDimension};
use std::collections::BTreeMap;

pub(super) type Balances = BTreeMap<UsageDimension, BudgetBalance>;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BudgetState {
    pub(super) scopes: BTreeMap<BudgetScope, Balances>,
    pub(super) reservations: BTreeMap<(BudgetScope, String), Reservation>,
}

impl BudgetState {
    pub fn new() -> Self {
        Self::default()
    }
}
