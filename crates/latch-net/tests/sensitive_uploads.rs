use latch_net::*;
#[path = "support/observed_connection.rs"]
mod observed_connection;
use observed_connection::connection;

#[test]
fn recognizes_a_secret_read_followed_by_an_upload() {
    let mut monitor = ExfiltrationMonitor::default();
    assert!(monitor.observe_read("/project/.env", 120, 100));
    assert!(!monitor.observe_read("/project/src/main.rs", 4000, 100));

    // The canonical leak: a secret is read, then something small goes to a
    // destination the policy does not name.
    assert_eq!(
        monitor.assess(&connection(200, 110), false, 110),
        Response::RequireApproval
    );
    // The same upload to a destination the policy names is ordinary work.
    assert_eq!(
        monitor.assess(&connection(200, 110), true, 110),
        Response::Allow
    );
    // And once the window has passed, the correlation is gone.
    assert_eq!(
        monitor.assess(&connection(200, 200), false, 200),
        Response::Allow
    );
}

#[test]
fn markers_are_matched_case_insensitively_anywhere_in_the_path() {
    let monitor = ExfiltrationMonitor::new(vec!["Secret".into()], 30);
    assert!(monitor.is_sensitive("/etc/SECRETS/db"));
    assert!(monitor.is_sensitive("app.secret.yaml"));
    assert!(!monitor.is_sensitive("/project/readme.md"));
}
