#![allow(dead_code)]
use latch_net::{Connection, Destination, DomainName, Host, Protocol};

pub fn connection(bytes_sent: u64, at: u64) -> Connection {
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
