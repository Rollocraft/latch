use std::net::{IpAddr, Ipv4Addr};

/// Addresses that are not a globally routable destination: the host itself,
/// its own networks, and every range reserved for something other than public
/// traffic. A name resolving here is the shape of a rebinding attack, and
/// 169.254.169.254 in particular is the cloud metadata service.
///
/// Reserved ranges that merely happen to be unroutable - documentation,
/// benchmarking, and the 240/4 block - count too. Refusing them costs nothing
/// real and keeps the answer to "may this connection leave" one question
/// rather than two.
pub fn is_private(address: &IpAddr) -> bool {
    match address {
        IpAddr::V4(v4) => is_private_v4(v4),
        IpAddr::V6(v6) => {
            let segments = v6.segments();
            // Judged before any v4 conversion: ::1 embeds 0.0.0.1, which says
            // nothing about whether ::1 is a loopback address.
            if v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_multicast()
                || segments[0] & 0xfe00 == 0xfc00 // unique local
                || segments[0] & 0xffc0 == 0xfe80
            // link local
            {
                return true;
            }
            match v6.to_ipv4_mapped() {
                Some(mapped) => is_private_v4(&mapped),
                // The deprecated ::a.b.c.d form is never a global destination.
                None => v6.to_ipv4().is_some(),
            }
        }
    }
}

fn is_private_v4(address: &Ipv4Addr) -> bool {
    let [a, b, ..] = address.octets();
    address.is_private()
        || address.is_loopback()
        || address.is_link_local()
        || address.is_broadcast()
        || address.is_documentation()
        || address.is_unspecified()
        || address.is_multicast()
        || (a == 100 && (64..=127).contains(&b)) // carrier-grade NAT
        || (a == 192 && b == 0) // protocol assignments
        || (a == 198 && (18..=19).contains(&b)) // benchmarking
        || a >= 240 // reserved
}

/// Named for symmetry with [`is_private`]; a loopback v6 address is not a v4
/// address and must not be judged as one.
pub fn is_loopback(address: &IpAddr) -> bool {
    match address {
        IpAddr::V4(v4) => v4.is_loopback(),
        IpAddr::V6(v6) => v6.is_loopback(),
    }
}
