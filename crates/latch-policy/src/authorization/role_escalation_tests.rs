use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn matching_abac_and_role_named_attributes_cannot_grant_roles() {
    let profile = profile(
        Operation::ManageIdentity,
        AttributePredicate::Equals {
            attribute: ResourceAttribute::Team,
            value: "OrganizationAdmin".into(),
        },
    );
    let request = AuthorizationRequest::new(
        Operation::ManageIdentity,
        ResourceAttributes::new(
            "tenant-a",
            Some("OrganizationAdmin".into()),
            Some("OrganizationAdmin".into()),
            Some("OrganizationAdmin".into()),
        )
        .unwrap(),
        None,
    )
    .unwrap();
    for roles in [
        vec![],
        vec![Role::Developer],
        vec![Role::Auditor, Role::Approver],
    ] {
        assert_eq!(
            authorize(&principal(roles), &request, &profile, 10),
            AuthorizationDecision::Denied(AuthorizationDenial::MissingRole {
                operation: Operation::ManageIdentity,
            })
        );
    }
}
