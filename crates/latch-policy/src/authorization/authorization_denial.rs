use super::{Operation, ResourceAttribute};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDenial {
    TenantMismatch,
    PrincipalNotYetValid,
    PrincipalExpired,
    MissingRole {
        operation: Operation,
    },
    SelfApproval,
    MissingApprovalOwner,
    OwnerApproval,
    MissingAttribute {
        constraint_index: usize,
        attribute: ResourceAttribute,
    },
    ConstraintNotSatisfied {
        constraint_index: usize,
        attribute: ResourceAttribute,
    },
}

impl fmt::Display for AuthorizationDenial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TenantMismatch => f.write_str("resource tenant differs from principal tenant"),
            Self::PrincipalNotYetValid => f.write_str("principal is not yet valid"),
            Self::PrincipalExpired => f.write_str("principal has expired"),
            Self::MissingRole { operation } => {
                write!(f, "no authenticated role permits {operation:?}")
            }
            Self::SelfApproval => f.write_str("requesters cannot approve their own actions"),
            Self::MissingApprovalOwner => {
                f.write_str("owner is required when owner approval is forbidden")
            }
            Self::OwnerApproval => f.write_str("permission profile forbids approval by the owner"),
            Self::MissingAttribute {
                constraint_index,
                attribute,
            } => write!(f, "constraint {constraint_index} requires {attribute:?}"),
            Self::ConstraintNotSatisfied {
                constraint_index,
                attribute,
            } => write!(f, "constraint {constraint_index} rejected {attribute:?}"),
        }
    }
}
