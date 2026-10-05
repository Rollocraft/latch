use latch_net::*;

#[test]
fn private_ranges_cover_the_addresses_rebinding_aims_at() {
    for value in [
        "127.0.0.1",
        "10.1.2.3",
        "192.168.1.1",
        "172.16.0.1",
        "169.254.169.254",
        "100.64.0.1",
        "0.0.0.0",
        "::1",
        "fd00::1",
        "fe80::1",
        "::ffff:127.0.0.1",
    ] {
        assert!(is_private(&value.parse().unwrap()), "{value} judged public");
    }
    for value in ["1.1.1.1", "140.82.121.4", "2606:4700::1111"] {
        assert!(
            !is_private(&value.parse().unwrap()),
            "{value} judged private"
        );
    }
    // Reserved but not private: refused all the same, because none of them
    // is a place real traffic goes.
    for value in [
        "192.0.2.1",
        "198.51.100.1",
        "203.0.113.1",
        "198.18.0.1",
        "240.0.0.1",
    ] {
        assert!(is_private(&value.parse().unwrap()), "{value} judged public");
    }
}
