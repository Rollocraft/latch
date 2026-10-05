use crate::error::invalid;
use crate::{AgentDescriptor, IdentityBinding, ManifestError, RequestedCapability};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const SCHEMA_VERSION: u32 = 1;
pub const MAX_MANIFEST_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_CAPABILITIES: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ManifestInput")]
pub struct AgentManifest {
    pub(crate) schema_version: u32,
    pub(crate) agent: AgentDescriptor,
    pub(crate) identity: IdentityBinding,
    pub(crate) requires: BTreeSet<RequestedCapability>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestInput {
    schema_version: u32,
    agent: AgentDescriptor,
    identity: IdentityBinding,
    requires: Vec<RequestedCapability>,
}

impl TryFrom<ManifestInput> for AgentManifest {
    type Error = ManifestError;
    fn try_from(value: ManifestInput) -> Result<Self, Self::Error> {
        Self::new(
            value.schema_version,
            value.agent,
            value.identity,
            value.requires,
        )
    }
}

impl AgentManifest {
    pub fn new(
        schema_version: u32,
        agent: AgentDescriptor,
        identity: IdentityBinding,
        requires: Vec<RequestedCapability>,
    ) -> Result<Self, ManifestError> {
        if schema_version != SCHEMA_VERSION {
            return Err(ManifestError::UnsupportedSchemaVersion(schema_version));
        }
        if requires.len() > MAX_CAPABILITIES {
            return Err(ManifestError::TooManyCapabilities);
        }
        let mut capabilities = BTreeSet::new();
        for capability in requires {
            if capability.environment() != identity.environment() {
                return Err(invalid(
                    "requires.environment",
                    "must equal the identity binding environment",
                ));
            }
            if !capabilities.insert(capability) {
                return Err(ManifestError::DuplicateCapability);
            }
        }
        Ok(Self {
            schema_version,
            agent,
            identity,
            requires: capabilities,
        })
    }
}
