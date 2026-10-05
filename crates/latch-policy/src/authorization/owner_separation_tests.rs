use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn owner_separation_is_configurable_and_defaults_to_required() {
    for role in [Role::TeamLead, Role::OrganizationAdmin, Role::Approver] {
        for owner in [None, Some("reviewer"), Some("other-owner")] {
            let request = AuthorizationRequest::new(
                Operation::ApproveAction,
                resource("tenant-a", owner),
                Some("requester".into()),
            )
            .unwrap();
            let decision = authorize(
                &principal([role]),
                &request,
                &PermissionProfile::default(),
                10,
            );
            match owner {
                None => assert_eq!(
                    decision,
                    AuthorizationDecision::Denied(AuthorizationDenial::MissingApprovalOwner)
                ),
                Some("reviewer") => assert_eq!(
                    decision,
                    AuthorizationDecision::Denied(AuthorizationDenial::OwnerApproval)
                ),
                _ => assert!(decision.is_allowed()),
            }
            let profile = PermissionProfile::new(vec![], OwnerApprovalPolicy::Allow).unwrap();
            assert!(authorize(&principal([role]), &request, &profile, 10).is_allowed());
        }
    }
}
