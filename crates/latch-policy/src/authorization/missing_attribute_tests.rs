use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn missing_attributes_fail_closed_for_every_predicate() {
    for attribute in [
        ResourceAttribute::Owner,
        ResourceAttribute::Team,
        ResourceAttribute::Environment,
    ] {
        for predicate in [
            AttributePredicate::Equals {
                attribute,
                value: "reviewer".into(),
            },
            AttributePredicate::In {
                attribute,
                values: vec!["reviewer".into()],
            },
            AttributePredicate::EqualsSubject { attribute },
        ] {
            let request = AuthorizationRequest::new(
                Operation::ReadAudit,
                ResourceAttributes::new("tenant-a", None, None, None).unwrap(),
                None,
            )
            .unwrap();
            assert_eq!(
                authorize(
                    &principal([Role::OrganizationAdmin]),
                    &request,
                    &profile(Operation::ReadAudit, predicate),
                    10
                ),
                AuthorizationDecision::Denied(AuthorizationDenial::MissingAttribute {
                    constraint_index: 0,
                    attribute
                })
            );
        }
    }
}
