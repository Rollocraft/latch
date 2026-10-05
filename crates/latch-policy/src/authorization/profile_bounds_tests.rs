use super::*;

#[test]
fn invalid_validity_windows_and_unbounded_profiles_are_rejected() {
    for (not_before, expires_at) in [(0, 0), (20, 10), (u64::MAX, u64::MAX)] {
        assert_eq!(
            AuthenticatedPrincipal::from_trusted_identity(
                "tenant",
                "subject",
                [],
                not_before,
                expires_at
            ),
            Err(AuthorizationValidationError::InvalidValidityWindow)
        );
    }
    for values in [
        vec![],
        vec!["a".into(), "a".into()],
        (0..=MAX_AUTHORIZATION_SET_VALUES)
            .map(|value| value.to_string())
            .collect(),
    ] {
        assert_eq!(
            PermissionProfile::new(
                vec![PermissionConstraint {
                    operation: Operation::ReadAudit,
                    predicate: AttributePredicate::In {
                        attribute: ResourceAttribute::Team,
                        values
                    },
                }],
                OwnerApprovalPolicy::Forbid
            ),
            Err(AuthorizationValidationError::InvalidPredicateSet)
        );
    }
    let constraint = PermissionConstraint {
        operation: Operation::ReadAudit,
        predicate: AttributePredicate::EqualsSubject {
            attribute: ResourceAttribute::Owner,
        },
    };
    assert!(
        PermissionProfile::new(
            vec![constraint.clone(); MAX_AUTHORIZATION_CONSTRAINTS],
            OwnerApprovalPolicy::Forbid
        )
        .is_ok()
    );
    assert_eq!(
        PermissionProfile::new(
            vec![constraint; MAX_AUTHORIZATION_CONSTRAINTS + 1],
            OwnerApprovalPolicy::Forbid
        ),
        Err(AuthorizationValidationError::TooManyConstraints)
    );
}
