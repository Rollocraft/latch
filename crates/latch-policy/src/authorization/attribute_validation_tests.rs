use super::*;

#[test]
fn invalid_identity_resource_and_predicate_attributes_are_rejected() {
    for invalid in [
        "".to_owned(),
        " ".into(),
        "two words".into(),
        "value\n".into(),
        "\0".into(),
        "x".repeat(MAX_AUTHORIZATION_ATTRIBUTE_BYTES + 1),
    ] {
        assert_eq!(
            AuthenticatedPrincipal::from_trusted_identity(&invalid, "subject", [], 0, 1),
            Err(AuthorizationValidationError::InvalidAttribute(
                AuthorizationField::Tenant
            ))
        );
        assert_eq!(
            AuthenticatedPrincipal::from_trusted_identity("tenant", &invalid, [], 0, 1),
            Err(AuthorizationValidationError::InvalidAttribute(
                AuthorizationField::Subject
            ))
        );
        assert_eq!(
            ResourceAttributes::new(&invalid, None, None, None),
            Err(AuthorizationValidationError::InvalidAttribute(
                AuthorizationField::Tenant
            ))
        );
        for (owner, team, environment, field) in [
            (Some(invalid.clone()), None, None, AuthorizationField::Owner),
            (None, Some(invalid.clone()), None, AuthorizationField::Team),
            (
                None,
                None,
                Some(invalid.clone()),
                AuthorizationField::Environment,
            ),
        ] {
            assert_eq!(
                ResourceAttributes::new("tenant", owner, team, environment),
                Err(AuthorizationValidationError::InvalidAttribute(field))
            );
        }
        for predicate in [
            AttributePredicate::Equals {
                attribute: ResourceAttribute::Team,
                value: invalid.clone(),
            },
            AttributePredicate::In {
                attribute: ResourceAttribute::Team,
                values: vec![invalid],
            },
        ] {
            assert_eq!(
                PermissionProfile::new(
                    vec![PermissionConstraint {
                        operation: Operation::ReadAudit,
                        predicate
                    }],
                    OwnerApprovalPolicy::Forbid
                ),
                Err(AuthorizationValidationError::InvalidAttribute(
                    AuthorizationField::PredicateValue
                ))
            );
        }
    }
}
