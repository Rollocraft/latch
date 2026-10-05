use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn subject_predicate_uses_authenticated_subject_and_exact_matching() {
    let profile = profile(
        Operation::ReadSession,
        AttributePredicate::EqualsSubject {
            attribute: ResourceAttribute::Owner,
        },
    );
    for (owner, allowed) in [
        ("reviewer", true),
        ("Reviewer", false),
        ("reviewer-extra", false),
    ] {
        let request = AuthorizationRequest::new(
            Operation::ReadSession,
            resource("tenant-a", Some(owner)),
            None,
        )
        .unwrap();
        assert_eq!(
            authorize(&principal([Role::Developer]), &request, &profile, 10).is_allowed(),
            allowed
        );
    }
}
