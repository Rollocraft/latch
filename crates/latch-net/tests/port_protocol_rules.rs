#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

use std::collections::BTreeSet;

#[test]
fn ports_and_protocols_narrow_a_rule() {
    let policy = Policy::new(vec![
        Rule {
            id: "https-only".into(),
            pattern: Pattern::parse("api.test").unwrap(),
            ports: Some(BTreeSet::from([443])),
            protocol: Some(Protocol::Tcp),
            effect: Effect::Allow,
        },
        rule("deny-rest", "*", Effect::Deny),
    ])
    .unwrap();
    let mut ledger = Ledger::new();
    resolved(&mut ledger, &["api.test"], &["172.217.16.142"], 100);

    let allowed = Destination {
        host: Host::Name(name("api.test")),
        port: 443,
        protocol: Protocol::Tcp,
    };
    assert_eq!(
        policy.evaluate(&allowed, &ledger, 110).outcome,
        Outcome::Allow
    );
    for (port, protocol) in [(80, Protocol::Tcp), (443, Protocol::Udp)] {
        let destination = Destination {
            host: Host::Name(name("api.test")),
            port,
            protocol,
        };
        assert_eq!(
            policy.evaluate(&destination, &ledger, 110).outcome,
            Outcome::Deny,
            "{port}/{protocol:?}"
        );
    }
}
