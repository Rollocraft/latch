use super::validate_attributes::validate_attribute;
use super::{AuthorizationField, AuthorizationValidationError, Operation, ResourceAttributes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRequest {
    operation: Operation,
    pub(super) resource: ResourceAttributes,
    approval_requester: Option<String>,
}

impl AuthorizationRequest {
    pub fn new(
        operation: Operation,
        resource: ResourceAttributes,
        approval_requester: Option<String>,
    ) -> Result<Self, AuthorizationValidationError> {
        if let Some(requester) = &approval_requester {
            validate_attribute(requester, AuthorizationField::ApprovalRequester)?;
        }
        match (operation, approval_requester.is_some()) {
            (Operation::ApproveAction, false) => {
                return Err(AuthorizationValidationError::MissingApprovalRequester);
            }
            (Operation::ApproveAction, true) | (_, false) => {}
            (_, true) => return Err(AuthorizationValidationError::UnexpectedApprovalRequester),
        }
        Ok(Self {
            operation,
            resource,
            approval_requester,
        })
    }

    pub fn operation(&self) -> Operation {
        self.operation
    }

    pub fn resource(&self) -> &ResourceAttributes {
        &self.resource
    }

    pub fn approval_requester(&self) -> Option<&str> {
        self.approval_requester.as_deref()
    }
}
