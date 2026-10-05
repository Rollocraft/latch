use super::{AuthenticatedPrincipal, AuthorizationDenial, AuthorizationRequest, PermissionProfile};

pub(super) fn check_constraints(
    principal: &AuthenticatedPrincipal,
    request: &AuthorizationRequest,
    profile: &PermissionProfile,
) -> Result<usize, AuthorizationDenial> {
    use AuthorizationDenial::*;
    let mut constraints_checked = 0;
    for (constraint_index, constraint) in profile.constraints().iter().enumerate() {
        if constraint.operation != request.operation() {
            continue;
        }
        let attribute = constraint.predicate.attribute();
        let actual = request
            .resource()
            .attribute(attribute)
            .ok_or(MissingAttribute {
                constraint_index,
                attribute,
            })?;
        if !constraint.predicate.matches(actual, principal) {
            return Err(ConstraintNotSatisfied {
                constraint_index,
                attribute,
            });
        }
        constraints_checked += 1;
    }
    Ok(constraints_checked)
}
