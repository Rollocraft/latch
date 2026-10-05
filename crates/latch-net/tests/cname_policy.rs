#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn a_chain_is_only_as_allowed_as_its_least_allowed_link() {
    let policy = Policy::new(vec![
        rule("allow-cdn", "*.example.com", Effect::Allow),
        rule("allow-assets", "*.assets.test", Effect::Allow),
        rule("deny-blocked", "*.blocked.test", Effect::Deny),
        rule("deny-rest", "*", Effect::Deny),
    ])
    .unwrap();
    let mut ledger = Ledger::new();

    resolved(
        &mut ledger,
        &["cdn.example.com", "edge.assets.test"],
        &["151.101.1.140"],
        100,
    );
    assert_eq!(
        policy
            .evaluate(&https(Host::Name(name("cdn.example.com"))), &ledger, 110)
            .outcome,
        Outcome::Allow
    );

    // The same first name, now aliased into a denied domain.
    resolved(
        &mut ledger,
        &["cdn.example.com", "hidden.blocked.test"],
        &["151.101.65.140"],
        120,
    );
    let decision = policy.evaluate(&https(Host::Name(name("cdn.example.com"))), &ledger, 130);
    assert_eq!(
        (decision.outcome, decision.reason),
        (Outcome::Deny, Reason::ExplicitDeny),
        "a denied CNAME target denies the connection that reached it"
    );
    assert_eq!(
        decision.names,
        vec![name("cdn.example.com"), name("hidden.blocked.test")]
    );
}

#[test]
fn an_address_reachable_two_ways_takes_the_stricter_answer() {
    let policy = Policy::new(vec![
        rule("allow-good", "good.test", Effect::Allow),
        rule("deny-bad", "bad.test", Effect::Deny),
        rule("deny-rest", "*", Effect::Deny),
    ])
    .unwrap();
    let mut ledger = Ledger::new();
    resolved(&mut ledger, &["good.test"], &["104.16.132.229"], 100);
    resolved(&mut ledger, &["bad.test"], &["104.16.132.229"], 100);

    let decision = policy.evaluate(
        &https(Host::Address("104.16.132.229".parse().unwrap())),
        &ledger,
        110,
    );
    assert_eq!(
        decision.outcome,
        Outcome::Deny,
        "sharing an address with a denied name is enough to refuse"
    );
}
