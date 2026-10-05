use super::*;

pub(super) const ROLES: [Role; 6] = [
    Role::Developer,
    Role::TeamLead,
    Role::SecurityAdmin,
    Role::OrganizationAdmin,
    Role::Auditor,
    Role::Approver,
];
pub(super) const OPERATIONS: [Operation; 7] = [
    Operation::ReadPolicy,
    Operation::ReadSession,
    Operation::ReadAudit,
    Operation::ManagePolicy,
    Operation::ApproveAction,
    Operation::ManageIdentity,
    Operation::KillSession,
];

pub(super) fn principal(roles: impl IntoIterator<Item = Role>) -> AuthenticatedPrincipal {
    AuthenticatedPrincipal::from_trusted_identity("tenant-a", "reviewer", roles, 10, 20).unwrap()
}

pub(super) fn resource(tenant: &str, owner: Option<&str>) -> ResourceAttributes {
    ResourceAttributes::new(
        tenant,
        owner.map(String::from),
        Some("platform".into()),
        Some("staging".into()),
    )
    .unwrap()
}

pub(super) fn request(operation: Operation) -> AuthorizationRequest {
    AuthorizationRequest::new(
        operation,
        resource("tenant-a", Some("owner")),
        (operation == Operation::ApproveAction).then(|| "requester".into()),
    )
    .unwrap()
}

pub(super) fn profile(operation: Operation, predicate: AttributePredicate) -> PermissionProfile {
    PermissionProfile::new(
        vec![PermissionConstraint {
            operation,
            predicate,
        }],
        OwnerApprovalPolicy::Forbid,
    )
    .unwrap()
}
