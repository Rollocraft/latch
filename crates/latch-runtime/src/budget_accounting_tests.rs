use super::*;
use std::sync::Arc;

fn scope(organization: &str) -> BudgetScope {
    BudgetScope::new(organization.into(), "s1".into()).unwrap()
}
fn values(items: &[(&str, u64)]) -> BTreeMap<String, u64> {
    items.iter().map(|(k, v)| ((*k).into(), *v)).collect()
}

#[test]
fn failed_multi_budget_charge_has_no_partial_effects() {
    let ledger = BudgetLedger::new();
    let limits = values(&[("bytes", 10), ("requests", 1)]);
    ledger
        .charge(
            &scope("acme"),
            &limits,
            &values(&[("bytes", 2), ("requests", 1)]),
        )
        .unwrap();
    let before = ledger.usage(&scope("acme")).unwrap();
    assert_eq!(
        ledger.charge(
            &scope("acme"),
            &limits,
            &values(&[("bytes", 2), ("requests", 1)])
        ),
        Err(BudgetError::Exhausted("requests".into()))
    );
    assert_eq!(ledger.usage(&scope("acme")).unwrap(), before);
}

#[test]
fn concurrent_charges_cannot_overspend() {
    let ledger = Arc::new(BudgetLedger::new());
    let handles: Vec<_> = (0..32)
        .map(|_| {
            let ledger = Arc::clone(&ledger);
            std::thread::spawn(move || {
                ledger
                    .charge(
                        &scope("acme"),
                        &values(&[("requests", 10)]),
                        &values(&[("requests", 1)]),
                    )
                    .is_ok()
            })
        })
        .collect();
    let successes = handles
        .into_iter()
        .filter_map(|h| h.join().unwrap().then_some(()))
        .count();
    assert_eq!(successes, 10);
    assert_eq!(ledger.usage(&scope("acme")).unwrap()["requests"], 10);
}

#[test]
fn tenant_scopes_do_not_share_usage() {
    let ledger = BudgetLedger::new();
    for tenant in ["acme", "other"] {
        ledger
            .charge(
                &scope(tenant),
                &values(&[("requests", 1)]),
                &values(&[("requests", 1)]),
            )
            .unwrap();
    }
}

#[test]
fn missing_cost_and_overflow_fail_closed() {
    let ledger = BudgetLedger::new();
    let limits = values(&[("bytes", u64::MAX)]);
    assert_eq!(
        ledger.charge(&scope("acme"), &limits, &values(&[])),
        Err(BudgetError::MissingCost("bytes".into()))
    );
    ledger.charge(&scope("acme"), &limits, &limits).unwrap();
    assert_eq!(
        ledger.charge(&scope("acme"), &limits, &values(&[("bytes", 1)])),
        Err(BudgetError::Exhausted("bytes".into()))
    );
    assert_eq!(ledger.usage(&scope("acme")).unwrap()["bytes"], u64::MAX);
}
