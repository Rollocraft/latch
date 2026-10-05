use super::validate_attributes::validate_attribute;
use super::{
    AuthenticatedPrincipal, AuthorizationField, AuthorizationValidationError,
    MAX_AUTHORIZATION_SET_VALUES, ResourceAttribute,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributePredicate {
    Equals {
        attribute: ResourceAttribute,
        value: String,
    },
    In {
        attribute: ResourceAttribute,
        values: Vec<String>,
    },
    EqualsSubject {
        attribute: ResourceAttribute,
    },
}

impl AttributePredicate {
    pub(super) fn validate(&self) -> Result<(), AuthorizationValidationError> {
        match self {
            Self::Equals { value, .. } => {
                validate_attribute(value, AuthorizationField::PredicateValue)
            }
            Self::In { values, .. } => {
                if values.is_empty() || values.len() > MAX_AUTHORIZATION_SET_VALUES {
                    return Err(AuthorizationValidationError::InvalidPredicateSet);
                }
                let mut distinct = BTreeSet::new();
                for value in values {
                    validate_attribute(value, AuthorizationField::PredicateValue)?;
                    if !distinct.insert(value) {
                        return Err(AuthorizationValidationError::InvalidPredicateSet);
                    }
                }
                Ok(())
            }
            Self::EqualsSubject { .. } => Ok(()),
        }
    }

    pub(super) fn attribute(&self) -> ResourceAttribute {
        match self {
            Self::Equals { attribute, .. }
            | Self::In { attribute, .. }
            | Self::EqualsSubject { attribute } => *attribute,
        }
    }

    pub(super) fn matches(&self, actual: &str, principal: &AuthenticatedPrincipal) -> bool {
        match self {
            Self::Equals { value, .. } => actual == value,
            Self::In { values, .. } => values.iter().any(|value| value == actual),
            Self::EqualsSubject { .. } => actual == principal.subject(),
        }
    }
}
