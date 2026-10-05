#![allow(dead_code)]

pub use latch_net::*;

pub fn name(value: &str) -> DomainName {
    DomainName::new(value).unwrap()
}

pub fn rule(id: &str, pattern: &str, effect: Effect) -> Rule {
    Rule {
        id: id.into(),
        pattern: Pattern::parse(pattern).unwrap(),
        ports: None,
        protocol: None,
        effect,
    }
}

/// A representative egress policy: named destinations allowed, everything else denied.
pub fn policy() -> Policy {
    Policy::new(vec![
        rule("allow-github", "github.com", Effect::Allow),
        rule("allow-npm", "npmjs.org", Effect::Allow),
        rule("ask-unknown", "*.unknown.dev", Effect::Ask),
        rule("deny-rest", "*", Effect::Deny),
    ])
    .unwrap()
}

pub fn https(host: Host) -> Destination {
    Destination {
        host,
        port: 443,
        protocol: Protocol::Tcp,
    }
}

pub fn resolved(ledger: &mut Ledger, chain: &[&str], addresses: &[&str], now: u64) {
    ledger
        .record(
            chain.iter().map(|n| name(n)).collect(),
            addresses.iter().map(|a| a.parse().unwrap()).collect(),
            now,
            300,
        )
        .unwrap();
}
