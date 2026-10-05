//! Tenant-scoped role and attribute authorization.

mod attribute_predicates;
mod authenticated_principal;
mod authorization_decision;
mod authorization_denial;
mod authorization_request;
mod authorize_request;
mod check_approval;
mod check_constraints;
mod check_principal;
mod permission_constraints;
mod resource_attributes;
mod role_permissions;
mod validate_attributes;

pub use attribute_predicates::*;
pub use authenticated_principal::*;
pub use authorization_decision::*;
pub use authorization_denial::*;
pub use authorization_request::*;
pub use authorize_request::authorize;
pub use permission_constraints::*;
pub use resource_attributes::*;
pub use role_permissions::*;
pub use validate_attributes::{
    AuthorizationField, AuthorizationValidationError, MAX_AUTHORIZATION_ATTRIBUTE_BYTES,
    MAX_AUTHORIZATION_CONSTRAINTS, MAX_AUTHORIZATION_SET_VALUES,
};

#[cfg(test)]
mod approval_requester_tests;
#[cfg(test)]
mod attribute_validation_tests;
#[cfg(test)]
mod authorization_test_fixtures;
#[cfg(test)]
mod constraint_intersection_tests;
#[cfg(test)]
mod decision_explanation_tests;
#[cfg(test)]
mod missing_attribute_tests;
#[cfg(test)]
mod owner_separation_tests;
#[cfg(test)]
mod principal_validity_tests;
#[cfg(test)]
mod profile_bounds_tests;
#[cfg(test)]
mod role_escalation_tests;
#[cfg(test)]
mod role_matrix_tests;
#[cfg(test)]
mod self_approval_tests;
#[cfg(test)]
mod subject_matching_tests;
#[cfg(test)]
mod tenant_isolation_tests;
