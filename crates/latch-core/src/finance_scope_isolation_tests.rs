use super::finance_test_support::*;
use super::*;

#[test]
fn tenants_sessions_and_reservation_ids_are_independent() {
    let (mut state, first) = configured();
    let second = scope("other", "session");
    let third = scope("tenant", "other");
    for scope in [&second, &third] {
        state
            .configure_scope(scope.clone(), &amounts(100, 50_000))
            .unwrap();
    }
    state
        .reserve(&first, "same", &amounts(100, 50_000))
        .unwrap();
    for scope in [&second, &third] {
        assert_eq!(
            state.reservation_state(scope, "same"),
            Err(BudgetError::UnknownReservation)
        );
        assert_eq!(
            state
                .balance(scope, UsageDimension::ApiTokens)
                .unwrap()
                .available(),
            Ok(100)
        );
        state.reserve(scope, "same", &amounts(100, 50_000)).unwrap();
    }
    state.cancel(&first, "same").unwrap();
    assert_eq!(
        state
            .balance(&second, UsageDimension::ApiTokens)
            .unwrap()
            .reserved(),
        100
    );
    assert_eq!(first.tenant(), "tenant");
    assert_eq!(first.session(), "session");
}
