#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn the_policy_decides_by_name_once_the_runtime_has_resolved_it() {
    let policy = policy();
    let mut ledger = Ledger::new();
    resolved(&mut ledger, &["github.com"], &["140.82.121.4"], 100);
    resolved(&mut ledger, &["files.unknown.dev"], &["93.184.216.34"], 100);
    resolved(&mut ledger, &["unknown.dev"], &["93.184.216.35"], 100);
    resolved(&mut ledger, &["evil.test"], &["45.33.32.156"], 100);

    let cases = [
        ("github.com", Outcome::Allow, Reason::ExplicitAllow),
        ("files.unknown.dev", Outcome::Ask, Reason::ApprovalRequired),
        // A wildcard over subdomains does not cover the parent, which then
        // falls through to the closing deny.
        ("unknown.dev", Outcome::Deny, Reason::ExplicitDeny),
        ("evil.test", Outcome::Deny, Reason::ExplicitDeny),
    ];
    for (host, outcome, reason) in cases {
        let decision = policy.evaluate(&https(Host::Name(name(host))), &ledger, 110);
        assert_eq!(
            (decision.outcome, &decision.reason),
            (outcome, &reason),
            "{host}"
        );
    }

    // The address the allowed name resolved to is allowed by that name.
    let decision = policy.evaluate(
        &https(Host::Address("140.82.121.4".parse().unwrap())),
        &ledger,
        110,
    );
    assert_eq!(decision.outcome, Outcome::Allow);
    assert_eq!(decision.names, vec![name("github.com")]);
}

#[test]
fn an_address_nobody_resolved_is_refused_however_allowed_its_owner_is() {
    let policy = policy();
    let empty = Ledger::new();

    // 140.82.121.4 is github.com, which the policy allows by name. Dialling it
    // directly is exactly the bypass the name rule exists to prevent.
    let decision = policy.evaluate(
        &https(Host::Address("140.82.121.4".parse().unwrap())),
        &empty,
        110,
    );
    assert_eq!(decision.outcome, Outcome::Deny);
    assert_eq!(decision.reason, Reason::UnresolvedAddress);

    // And a name nobody looked up says nothing about where it points.
    let decision = policy.evaluate(&https(Host::Name(name("github.com"))), &empty, 110);
    assert_eq!(
        (decision.outcome, decision.reason),
        (Outcome::Deny, Reason::UnresolvedName)
    );
}
