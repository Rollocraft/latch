use crate::DomainName;
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    pub host: Host,
    pub port: u16,
    pub protocol: Protocol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Host {
    Name(DomainName),
    Address(IpAddr),
}

pub(crate) fn key_of(destination: &Destination) -> String {
    let host = match &destination.host {
        Host::Name(name) => name.to_string(),
        Host::Address(address) => address.to_string(),
    };
    format!("{host}:{}:{:?}", destination.port, destination.protocol)
}
