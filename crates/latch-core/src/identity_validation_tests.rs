use super::identity_test_support::*;
use super::*;

#[test]
fn identity_requires_attribution_and_valid_time() {
    let mut id = identity();
    assert!(id.validate_at(10).is_ok());
    assert!(id.validate_at(100).is_err());
    assert!(id.validate_at(9).is_err());
    id.owner.clear();
    assert_eq!(id.validate_at(20), Err(ValidationError::MissingField));
}

#[test]
fn identity_rejects_ambiguous_or_cross_tenant_uris() {
    for uri in [
        "agent://other/coder",
        "agent://acme/../coder",
        "agent://acme//coder",
        "agent://acme/coder?admin",
        "agent://acme/%2f",
    ] {
        let mut id = identity();
        id.id = uri.into();
        assert_eq!(id.validate_at(20), Err(ValidationError::InvalidIdentity));
    }
}
