use super::finance_test_support::*;
use super::*;

#[test]
fn reserve_is_atomic_across_dimensions() {
    let (mut state, scope) = configured();
    let before = snapshot(&state);
    assert_eq!(
        state.reserve(&scope, "r", &amounts(1, 50_001)),
        Err(BudgetError::LimitExceeded(UsageDimension::Refunds))
    );
    assert_eq!(state, before);
    state.reserve(&scope, "r", &amounts(100, 50_000)).unwrap();
    let before = snapshot(&state);
    assert_eq!(
        state.reserve(&scope, "next", &amounts(1, 0)),
        Err(BudgetError::LimitExceeded(UsageDimension::ApiTokens))
    );
    assert_eq!(state, before);
    assert_eq!(
        state
            .balance(&scope, UsageDimension::Refunds)
            .unwrap()
            .available(),
        Ok(0)
    );
}
