use latch_net::*;

#[test]
fn a_wildcard_grants_no_more_than_it_says() {
    let pattern = Pattern::parse("*.example.com").unwrap();
    assert!(pattern.matches(&DomainName::new("api.example.com").unwrap()));
    assert!(pattern.matches(&DomainName::new("a.b.example.com").unwrap()));
    assert!(!pattern.matches(&DomainName::new("example.com").unwrap()));
    assert!(!pattern.matches(&DomainName::new("notexample.com").unwrap()));
    assert!(!pattern.matches(&DomainName::new("example.com.evil.test").unwrap()));
    assert!(
        Pattern::parse("*")
            .unwrap()
            .matches(&DomainName::new("any.test").unwrap())
    );
    for value in ["a.*", "*x.com", "**", "*.*", ""] {
        assert!(Pattern::parse(value).is_err(), "{value:?} was accepted");
    }
}
