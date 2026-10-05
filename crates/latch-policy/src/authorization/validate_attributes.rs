use std::fmt;

pub const MAX_AUTHORIZATION_ATTRIBUTE_BYTES: usize = 256;
pub const MAX_AUTHORIZATION_CONSTRAINTS: usize = 64;
pub const MAX_AUTHORIZATION_SET_VALUES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationField {
    Tenant,
    Subject,
    Owner,
    Team,
    Environment,
    ApprovalRequester,
    PredicateValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationValidationError {
    InvalidAttribute(AuthorizationField),
    InvalidValidityWindow,
    MissingApprovalRequester,
    UnexpectedApprovalRequester,
    InvalidPredicateSet,
    TooManyConstraints,
}

impl fmt::Display for AuthorizationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAttribute(field) => {
                write!(f, "invalid authorization attribute: {field:?}")
            }
            Self::InvalidValidityWindow => f.write_str("validity requires not_before < expires_at"),
            Self::MissingApprovalRequester => {
                f.write_str("approval requires the original requester")
            }
            Self::UnexpectedApprovalRequester => {
                f.write_str("approval requester is only valid for ApproveAction")
            }
            Self::InvalidPredicateSet => {
                f.write_str("predicate set must be nonempty, bounded, and contain distinct values")
            }
            Self::TooManyConstraints => f.write_str("permission profile has too many constraints"),
        }
    }
}

impl std::error::Error for AuthorizationValidationError {}

pub(super) fn validate_attribute(
    value: &str,
    field: AuthorizationField,
) -> Result<(), AuthorizationValidationError> {
    if value.is_empty()
        || value.len() > MAX_AUTHORIZATION_ATTRIBUTE_BYTES
        || value
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(AuthorizationValidationError::InvalidAttribute(field));
    }
    Ok(())
}
