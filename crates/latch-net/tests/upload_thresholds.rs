use latch_net::*;
#[path = "support/observed_connection.rs"]
mod observed_connection;
use observed_connection::connection;

#[test]
fn volume_alone_escalates_without_any_secret_read() {
    let monitor = ExfiltrationMonitor::default();
    for (bytes, expected) in [
        (1, Response::Allow),
        (1 << 20, Response::Warn),
        (10 << 20, Response::RequireApproval),
        (100 << 20, Response::Freeze),
        (u64::MAX, Response::Freeze),
    ] {
        assert_eq!(
            monitor.assess(&connection(bytes, 10), true, 10),
            expected,
            "{bytes} bytes"
        );
    }
}
