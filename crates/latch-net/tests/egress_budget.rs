#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn the_egress_budget_bounds_volume_destinations_and_requests() {
    let mut budget = Budget::new(Limits {
        uploaded_bytes: 5 << 20,
        destinations: 2,
        requests: 3,
    });
    let to = |host: &str| https(Host::Name(name(host)));

    budget.charge(&to("a.test"), 1 << 20).unwrap();
    budget.charge(&to("b.test"), 1 << 20).unwrap();
    // A third distinct destination is refused, and nothing is recorded for it.
    assert_eq!(
        budget.charge(&to("c.test"), 1),
        Err(BudgetError::TooManyDestinations)
    );
    assert_eq!(budget.destinations(), 2);
    assert_eq!(budget.requests(), 2);

    // Revisiting a destination already paid for is fine until the request
    // count runs out.
    budget.charge(&to("a.test"), 1).unwrap();
    assert_eq!(
        budget.charge(&to("a.test"), 1),
        Err(BudgetError::TooManyRequests)
    );

    let mut budget = Budget::new(Limits {
        uploaded_bytes: 1000,
        destinations: 10,
        requests: 10,
    });
    budget.charge(&to("a.test"), 900).unwrap();
    assert_eq!(
        budget.charge(&to("a.test"), 200),
        Err(BudgetError::UploadExhausted)
    );
    assert_eq!(budget.uploaded(), 900, "a refusal costs nothing");
    assert_eq!(
        budget.charge(&to("a.test"), u64::MAX),
        Err(BudgetError::UploadExhausted)
    );
}
