//! Egress decisions, exercised the way an agent would try to get around them:
//! by addressing a host directly, by re-pointing a name, by hiding a denied
//! domain behind a CNAME, and by leaving quietly with a lot of data.

#![allow(unused_imports)]
#[path = "support/egress_fixtures.rs"]
mod egress_fixtures;
use egress_fixtures::*;

#[test]
fn a_secret_read_turns_an_ordinary_upload_into_a_question() {
    let policy = policy();
    let mut ledger = Ledger::new();
    resolved(&mut ledger, &["files.unknown.dev"], &["93.184.216.34"], 100);
    let destination = https(Host::Name(name("files.unknown.dev")));

    let mut monitor = ExfiltrationMonitor::default();
    monitor.observe_read("/project/.env", 120, 100);

    let decision = policy.evaluate(&destination, &ledger, 110);
    assert_eq!(decision.outcome, Outcome::Ask);

    let connection = Connection {
        destination: destination.clone(),
        names: decision.names.clone(),
        bytes_sent: 2_000,
        bytes_received: 0,
        at: 110,
    };
    // Named outright by the policy would be familiar; reached through a
    // wildcard is not, which is where an unknown collector would sit.
    assert_eq!(
        monitor.assess(&connection, false, 110),
        Response::RequireApproval
    );

    let totals = totals(&[connection]);
    assert_eq!(totals.values().next(), Some(&(2_000, 0)));
}
