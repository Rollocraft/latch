use super::finance_test_support::*;
use super::*;

#[test]
fn identifiers_and_storage_are_bounded() {
    for id in [
        String::new(),
        " ".into(),
        "a b".into(),
        "a\n".into(),
        "é".into(),
        "x".repeat(MAX_SCOPE_BYTES + 1),
    ] {
        assert_eq!(
            BudgetScope::new(&id, "s"),
            Err(BudgetError::InvalidIdentifier)
        );
        assert_eq!(
            BudgetScope::new("t", &id),
            Err(BudgetError::InvalidIdentifier)
        );
        let (mut state, scope) = configured();
        let before = snapshot(&state);
        assert_eq!(
            state.reserve(&scope, &id, &amounts(0, 0)),
            Err(BudgetError::InvalidIdentifier)
        );
        assert_eq!(state, before);
    }
    assert!(BudgetScope::new(&"x".repeat(MAX_SCOPE_BYTES), "s").is_ok());
    let mut state = BudgetState::new();
    for index in 0..MAX_BUDGET_SCOPES {
        state
            .configure_scope(scope(&format!("t{index}"), "s"), &amounts(0, 0))
            .unwrap();
    }
    assert_eq!(
        state.configure_scope(scope("extra", "s"), &amounts(0, 0)),
        Err(BudgetError::ScopeCapacityReached)
    );
    let first = scope("t0", "s");
    for index in 0..MAX_RESERVATIONS {
        let id = format!("r{index}");
        state.reserve(&first, &id, &amounts(0, 0)).unwrap();
        state.cancel(&first, &id).unwrap();
    }
    assert_eq!(
        state.reserve(&first, "extra", &amounts(0, 0)),
        Err(BudgetError::ReservationCapacityReached)
    );
    assert_eq!(
        state.reserve(&first, "r0", &amounts(0, 0)),
        Err(BudgetError::DuplicateReservation)
    );
}
