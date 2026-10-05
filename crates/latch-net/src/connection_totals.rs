use crate::destination::key_of;
use crate::{Destination, DomainName};
use std::collections::BTreeMap;

/// A connection record, which is what an audit timeline and the exfiltration
/// monitor are built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    pub destination: Destination,
    pub names: Vec<DomainName>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub at: u64,
}

/// Per-destination totals for a session, as shown on its timeline.
pub fn totals(connections: &[Connection]) -> BTreeMap<String, (u64, u64)> {
    let mut totals: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for connection in connections {
        let entry = totals.entry(key_of(&connection.destination)).or_default();
        entry.0 += connection.bytes_sent;
        entry.1 += connection.bytes_received;
    }
    totals
}
