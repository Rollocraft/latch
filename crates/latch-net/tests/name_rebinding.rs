#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn re_pointing_a_name_does_not_carry_its_old_address_along() {
    let policy = policy();
    let mut ledger = Ledger::new();
    resolved(&mut ledger, &["github.com"], &["140.82.121.4"], 100);

    // The name is re-answered, now pointing at the cloud metadata service.
    resolved(&mut ledger, &["github.com"], &["169.254.169.254"], 150);

    let decision = policy.evaluate(&https(Host::Name(name("github.com"))), &ledger, 160);
    assert_eq!(decision.outcome, Outcome::Deny);
    assert_eq!(
        decision.reason,
        Reason::PrivateAddress("169.254.169.254".parse().unwrap()),
        "an allowed name resolving inward is still refused"
    );

    // The address from before the change is no longer usable either.
    let decision = policy.evaluate(
        &https(Host::Address("140.82.121.4".parse().unwrap())),
        &ledger,
        160,
    );
    assert_eq!(decision.reason, Reason::UnresolvedAddress);
}
