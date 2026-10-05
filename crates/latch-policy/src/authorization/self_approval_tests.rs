use super::authorization_test_fixtures::*;
use super::*;

#[test]
fn self_approval_is_unconditionally_denied_for_all_approval_roles() {
    for role in [Role::TeamLead, Role::OrganizationAdmin, Role::Approver] {
        for owner_approval in [OwnerApprovalPolicy::Allow, OwnerApprovalPolicy::Forbid] {
            let profile = PermissionProfile::new(vec![], owner_approval).unwrap();
            let request = AuthorizationRequest::new(
                Operation::ApproveAction,
                resource("tenant-a", None),
                Some("reviewer".into()),
            )
            .unwrap();
            assert_eq!(
                authorize(&principal([role]), &request, &profile, 10),
                AuthorizationDecision::Denied(AuthorizationDenial::SelfApproval)
            );
        }
    }
}
