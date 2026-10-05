#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn reaching_into_the_hosts_own_networks_needs_saying_so() {
    let rules = vec![rule("allow-internal", "internal.test", Effect::Allow)];
    let mut ledger = Ledger::new();
    resolved(&mut ledger, &["internal.test"], &["10.0.0.5"], 100);
    let destination = https(Host::Name(name("internal.test")));

    let closed = Policy::new(rules.clone()).unwrap();
    assert_eq!(
        closed.evaluate(&destination, &ledger, 110).reason,
        Reason::PrivateAddress("10.0.0.5".parse().unwrap())
    );

    let open = Policy::new(rules).unwrap().allowing_private_addresses();
    assert_eq!(
        open.evaluate(&destination, &ledger, 110).outcome,
        Outcome::Allow
    );
}
