use super::{AuthenticatedPrincipal, AuthorizationDenial, AuthorizationRequest, Role};

pub(super) fn check_principal(
    principal: &AuthenticatedPrincipal,
    request: &AuthorizationRequest,
    trusted_now: u64,
) -> Result<Role, AuthorizationDenial> {
    use AuthorizationDenial::*;
    if principal.tenant() != request.resource().tenant() {
        return Err(TenantMismatch);
    }
    if trusted_now < principal.not_before() {
        return Err(PrincipalNotYetValid);
    }
    if trusted_now >= principal.expires_at() {
        return Err(PrincipalExpired);
    }
    principal
        .roles()
        .iter()
        .copied()
        .find(|role| role.permits(request.operation()))
        .ok_or(MissingRole {
            operation: request.operation(),
        })
}
