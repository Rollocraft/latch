use super::{
    AttributePredicate, AuthorizationValidationError, MAX_AUTHORIZATION_CONSTRAINTS, Operation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionConstraint {
    pub operation: Operation,
    pub predicate: AttributePredicate,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OwnerApprovalPolicy {
    Allow,
    #[default]
    Forbid,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PermissionProfile {
    constraints: Vec<PermissionConstraint>,
    owner_approval: OwnerApprovalPolicy,
}

impl PermissionProfile {
    pub fn new(
        constraints: Vec<PermissionConstraint>,
        owner_approval: OwnerApprovalPolicy,
    ) -> Result<Self, AuthorizationValidationError> {
        if constraints.len() > MAX_AUTHORIZATION_CONSTRAINTS {
            return Err(AuthorizationValidationError::TooManyConstraints);
        }
        for constraint in &constraints {
            constraint.predicate.validate()?;
        }
        Ok(Self {
            constraints,
            owner_approval,
        })
    }

    pub fn constraints(&self) -> &[PermissionConstraint] {
        &self.constraints
    }

    pub fn owner_approval(&self) -> OwnerApprovalPolicy {
        self.owner_approval
    }
}
