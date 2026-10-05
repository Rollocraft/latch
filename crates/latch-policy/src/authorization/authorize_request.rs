use super::{
    AuthenticatedPrincipal, AuthorizationDecision, AuthorizationRequest, PermissionProfile,
};
use super::{
    check_approval::check_approval, check_constraints::check_constraints,
    check_principal::check_principal,
};

pub fn authorize(
    principal: &AuthenticatedPrincipal,
    request: &AuthorizationRequest,
    profile: &PermissionProfile,
    trusted_now: u64,
) -> AuthorizationDecision {
    let result = (|| {
        let role = check_principal(principal, request, trusted_now)?;
        check_approval(principal, request, profile)?;
        let constraints_checked = check_constraints(principal, request, profile)?;
        Ok(AuthorizationDecision::Allowed {
            role,
            constraints_checked,
        })
    })();
    result.unwrap_or_else(AuthorizationDecision::Denied)
}
