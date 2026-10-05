use super::{
    AuthenticatedPrincipal, AuthorizationDenial, AuthorizationRequest, Operation,
    OwnerApprovalPolicy, PermissionProfile, ResourceAttribute,
};

pub(super) fn check_approval(
    principal: &AuthenticatedPrincipal,
    request: &AuthorizationRequest,
    profile: &PermissionProfile,
) -> Result<(), AuthorizationDenial> {
    use AuthorizationDenial::*;
    if request.operation() != Operation::ApproveAction {
        return Ok(());
    }
    if request.approval_requester() == Some(principal.subject()) {
        return Err(SelfApproval);
    }
    if profile.owner_approval() == OwnerApprovalPolicy::Forbid {
        let owner = request
            .resource()
            .attribute(ResourceAttribute::Owner)
            .ok_or(MissingApprovalOwner)?;
        if owner == principal.subject() {
            return Err(OwnerApproval);
        }
    }
    Ok(())
}
