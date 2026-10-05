use super::{AuthorizationDenial, Role};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDecision {
    Allowed {
        role: Role,
        constraints_checked: usize,
    },
    Denied(AuthorizationDenial),
}

impl AuthorizationDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed { .. })
    }
}

impl fmt::Display for AuthorizationDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allowed {
                role,
                constraints_checked,
            } => write!(
                f,
                "allowed by {role:?}; tenant, validity, approval checks and {constraints_checked} ABAC constraints passed"
            ),
            Self::Denied(reason) => write!(f, "denied: {reason}"),
        }
    }
}
