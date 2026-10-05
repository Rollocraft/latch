use latch_net::*;

#[test]
fn names_are_normalized_and_ambiguous_ones_refused() {
    assert_eq!(
        DomainName::new("GitHub.Com.").unwrap().as_str(),
        "github.com"
    );
    assert_eq!(DomainName::new("localhost").unwrap().as_str(), "localhost");
    for value in [
        "", ".", "..", "a..b", "-a.com", "a-.com", "a_b.com", "a b.com",
    ] {
        assert!(DomainName::new(value).is_err(), "{value:?} was accepted");
    }
    for value in ["1.2.3.4", "::1", "10.0.0.1", "example.42"] {
        assert_eq!(
            DomainName::new(value),
            Err(NetworkError::AddressAsName),
            "{value:?}"
        );
    }
    assert_eq!(
        DomainName::new(&"a".repeat(64)),
        Err(NetworkError::InvalidLabel)
    );
}
