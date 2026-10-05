#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn a_resolution_stops_counting_when_it_expires() {
    let policy = policy();
    let mut ledger = Ledger::new();
    let address = Host::Address("140.82.121.4".parse().unwrap());
    ledger
        .record(
            vec![name("github.com")],
            vec!["140.82.121.4".parse().unwrap()],
            100,
            86_400,
        )
        .unwrap();

    assert_eq!(
        policy
            .evaluate(&https(address.clone()), &ledger, 100 + MAXIMUM_TTL - 1)
            .outcome,
        Outcome::Allow
    );
    // A server cannot pin an address for a day: the ttl is clamped.
    assert_eq!(
        policy
            .evaluate(&https(address.clone()), &ledger, 100 + MAXIMUM_TTL)
            .reason,
        Reason::UnresolvedAddress
    );

    ledger.forget_expired(100 + MAXIMUM_TTL);
    assert!(ledger.resolutions().is_empty());
}
