use super::finance_test_support::*;
use super::*;

#[test]
fn all_dimensions_have_explicit_units() {
    let mut state = BudgetState::new();
    let scope = scope("a", "s");
    let limits = [
        (UsageDimension::ApiTokens, UsageAmount::Units(1)),
        (UsageDimension::CloudSpend, UsageAmount::Money(eur(1))),
        (UsageDimension::GpuMilliseconds, UsageAmount::Units(1)),
        (UsageDimension::StorageBytes, UsageAmount::Units(1)),
        (UsageDimension::Requests, UsageAmount::Units(1)),
        (UsageDimension::Emails, UsageAmount::Units(1)),
        (UsageDimension::Refunds, UsageAmount::Money(eur(1))),
    ];
    state.configure_scope(scope.clone(), &limits).unwrap();
    state.reserve(&scope, "all", &limits).unwrap();
    state.begin(&scope, "all").unwrap();
    state
        .commit(&scope, "all", OperationOutcome::Applied)
        .unwrap();
    for (dimension, limit) in limits {
        let balance = state.balance(&scope, dimension).unwrap();
        assert_eq!(balance.limit(), limit);
        assert_eq!(balance.reserved(), 0);
        assert_eq!(balance.consumed(), 1);
        assert_eq!(balance.available(), Ok(0));
    }
}
