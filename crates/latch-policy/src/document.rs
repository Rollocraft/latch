//! Bounded JSON and YAML policy loading.

#[path = "bounded_document_value.rs"]
mod bounded_document_value;
#[path = "document_errors.rs"]
mod document_errors;
#[path = "load_policy_document.rs"]
mod load_policy_document;
#[path = "policy_document.rs"]
mod policy_document;
#[path = "visit_document_value.rs"]
mod visit_document_value;
#[path = "wire_argument_matcher.rs"]
mod wire_argument_matcher;
#[path = "wire_policy.rs"]
mod wire_policy;
#[path = "wire_rule.rs"]
mod wire_rule;

pub use document_errors::*;
pub use load_policy_document::load_policy_document;
pub use policy_document::PolicyDocument;
