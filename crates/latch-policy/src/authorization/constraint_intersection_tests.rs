use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn all_applicable_constraints_apply_even_to_administrators_with_multiple_roles() {
    let profile = PermissionProfile::new(
        vec![
            PermissionConstraint {
                operation: Operation::ReadAudit,
                predicate: AttributePredicate::Equals {
                    attribute: ResourceAttribute::Team,
                    value: "platform".into(),
                },
            },
            PermissionConstraint {
                operation: Operation::ReadAudit,
                predicate: AttributePredicate::In {
                    attribute: ResourceAttribute::Environment,
                    values: vec!["staging".into(), "development".into()],
                },
            },
            PermissionConstraint {
                operation: Operation::ManagePolicy,
                predicate: AttributePredicate::Equals {
                    attribute: ResourceAttribute::Environment,
                    value: "production".into(),
                },
            },
        ],
        OwnerApprovalPolicy::Forbid,
    )
    .unwrap();
    let principal = principal(ROLES);
    assert_eq!(
        authorize(&principal, &request(Operation::ReadAudit), &profile, 10),
        AuthorizationDecision::Allowed {
            role: Role::TeamLead,
            constraints_checked: 2,
        }
    );
    let mut audit_request = request(Operation::ReadAudit);
    audit_request.resource.environment = Some("production".into());
    assert_eq!(
        authorize(&principal, &audit_request, &profile, 10),
        AuthorizationDecision::Denied(AuthorizationDenial::ConstraintNotSatisfied {
            constraint_index: 1,
            attribute: ResourceAttribute::Environment,
        })
    );
    audit_request.resource.environment = Some("staging".into());
    audit_request.resource.team = Some("other-team".into());
    assert_eq!(
        authorize(&principal, &audit_request, &profile, 10),
        AuthorizationDecision::Denied(AuthorizationDenial::ConstraintNotSatisfied {
            constraint_index: 0,
            attribute: ResourceAttribute::Team,
        })
    );
}
