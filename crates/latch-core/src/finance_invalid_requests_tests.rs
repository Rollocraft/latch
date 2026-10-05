use super::finance_test_support::*;
use super::*;

#[test]
fn invalid_requests_preserve_all_state() {
    let (mut state, scope) = configured();
    let before = snapshot(&state);
    let cases = [
        (vec![], BudgetError::InvalidDimensionCount),
        (
            vec![(UsageDimension::ApiTokens, UsageAmount::Units(1)); 2],
            BudgetError::DuplicateDimension(UsageDimension::ApiTokens),
        ),
        (
            vec![(UsageDimension::ApiTokens, UsageAmount::Units(1)); 8],
            BudgetError::InvalidDimensionCount,
        ),
        (
            vec![(UsageDimension::Requests, UsageAmount::Units(1))],
            BudgetError::UnknownDimension(UsageDimension::Requests),
        ),
        (
            vec![(UsageDimension::Refunds, UsageAmount::Units(1))],
            BudgetError::UnitMismatch(UsageDimension::Refunds),
        ),
        (
            vec![(UsageDimension::ApiTokens, UsageAmount::Money(eur(1)))],
            BudgetError::UnitMismatch(UsageDimension::ApiTokens),
        ),
        (
            vec![
                (UsageDimension::ApiTokens, UsageAmount::Units(1)),
                (
                    UsageDimension::Refunds,
                    UsageAmount::Money(Money::new(Currency::new("USD").unwrap(), 0)),
                ),
            ],
            BudgetError::CurrencyMismatch(UsageDimension::Refunds),
        ),
    ];
    for (request, error) in cases {
        assert_eq!(state.reserve(&scope, "r", &request), Err(error));
        assert_eq!(state, before);
    }
    assert_eq!(
        state.configure_scope(scope.clone(), &amounts(200, 100_000)),
        Err(BudgetError::ScopeAlreadyConfigured)
    );
    assert_eq!(state, before);
}
