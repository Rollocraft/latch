use super::*;
use crate::{Connection, Destination, DomainName, Host, Protocol};

fn connection(bytes_sent: u64, at: u64) -> Connection {
    Connection {
        destination: Destination {
            host: Host::Name(DomainName::new("collector.test").unwrap()),
            port: 443,
            protocol: Protocol::Tcp,
        },
        names: vec![DomainName::new("collector.test").unwrap()],
        bytes_sent,
        bytes_received: 0,
        at,
    }
}

#[test]
fn forgetting_bounds_the_monitor_and_its_suspicion() {
    let mut monitor = ExfiltrationMonitor::default();
    monitor.observe_read("/app/credentials.json", 10, 100);
    monitor.observe_read("/app/.env", 10, 200);
    assert_eq!(monitor.recent_reads(220).len(), 1);
    monitor.forget_before(150);
    assert_eq!(monitor.reads.len(), 1);
    monitor.forget_before(250);
    assert_eq!(
        monitor.assess(&connection(10, 260), false, 260),
        Response::Allow
    );
}
