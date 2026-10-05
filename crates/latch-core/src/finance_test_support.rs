use super::*;

pub(super) fn eur(value: u64) -> Money {
    Money::new(Currency::new("EUR").unwrap(), value)
}

pub(super) fn scope(tenant: &str, session: &str) -> BudgetScope {
    BudgetScope::new(tenant, session).unwrap()
}

pub(super) fn amounts(tokens: u64, refunds: u64) -> [(UsageDimension, UsageAmount); 2] {
    [
        (UsageDimension::ApiTokens, UsageAmount::Units(tokens)),
        (UsageDimension::Refunds, UsageAmount::Money(eur(refunds))),
    ]
}

pub(super) fn configured() -> (BudgetState, BudgetScope) {
    let mut state = BudgetState::new();
    let scope = scope("tenant", "session");
    state
        .configure_scope(scope.clone(), &amounts(100, 50_000))
        .unwrap();
    (state, scope)
}

pub(super) fn snapshot(state: &BudgetState) -> BudgetState {
    BudgetState {
        scopes: state.scopes.clone(),
        reservations: state.reservations.clone(),
    }
}
