use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn tenant_isolation_precedes_time_roles_approval_and_abac() {
    let restrictive = profile(
        Operation::ApproveAction,
        AttributePredicate::Equals {
            attribute: ResourceAttribute::Environment,
            value: "production".into(),
        },
    );
    for roles in ROLES
        .map(|role| vec![role])
        .into_iter()
        .chain([vec![], ROLES.to_vec()])
    {
        for operation in OPERATIONS {
            let request = AuthorizationRequest::new(
                operation,
                resource("tenant-b", Some("reviewer")),
                (operation == Operation::ApproveAction).then(|| "reviewer".into()),
            )
            .unwrap();
            for now in [0, 10, 20, u64::MAX] {
                assert_eq!(
                    authorize(&principal(roles.clone()), &request, &restrictive, now),
                    AuthorizationDecision::Denied(AuthorizationDenial::TenantMismatch)
                );
            }
        }
    }
}
