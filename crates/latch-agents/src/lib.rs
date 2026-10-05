mod action_name;
mod agent_descriptor;
mod bound_manifest;
mod capability;
mod diff;
mod error;
mod identity;
mod identity_access;
mod identity_validation;
mod manifest;
mod manifest_access;
mod model_metadata;
mod resource_id;
mod resource_scope;
mod resource_uri_validation;

pub use action_name::ActionName;
pub use bound_manifest::BoundManifest;
pub use capability::RequestedCapability;
pub use diff::PermissionDiff;
pub use error::ManifestError;
pub use identity::IdentityBinding;
pub use manifest::{AgentManifest, MAX_CAPABILITIES, MAX_MANIFEST_BYTES, SCHEMA_VERSION};
pub use resource_id::ResourceId;
pub use resource_scope::{ResourceScope, ScopeKind};

#[cfg(test)]
mod tests;

pub use agent_descriptor::AgentDescriptor;
pub use model_metadata::ModelMetadata;

#[cfg(test)]
mod manifest_test_fixtures;
