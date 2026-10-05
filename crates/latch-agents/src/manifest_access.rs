use crate::{
    ActionName, AgentDescriptor, AgentManifest, BoundManifest, IdentityBinding, MAX_MANIFEST_BYTES,
    ManifestError, RequestedCapability, ResourceId,
};
use latch_core::AgentIdentity;
use std::collections::BTreeSet;

impl AgentManifest {
    pub fn from_json(input: &str) -> Result<Self, serde_json::Error> {
        if input.len() > MAX_MANIFEST_BYTES {
            return Err(<serde_json::Error as serde::de::Error>::custom(
                ManifestError::ManifestTooLarge,
            ));
        }
        serde_json::from_str(input)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }
    pub fn agent(&self) -> &AgentDescriptor {
        &self.agent
    }
    pub fn identity(&self) -> &IdentityBinding {
        &self.identity
    }
    pub fn requires(&self) -> &BTreeSet<RequestedCapability> {
        &self.requires
    }

    pub fn requests(&self, action: &ActionName, resource: &ResourceId, environment: &str) -> bool {
        self.requires
            .iter()
            .any(|capability| capability.requests(action, resource, environment))
    }

    pub fn bind(
        &self,
        runtime_identity: &AgentIdentity,
        now: u64,
    ) -> Result<BoundManifest, ManifestError> {
        BoundManifest::new(self.clone(), runtime_identity, now)
    }
}
