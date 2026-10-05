use crate::ManifestError;
use latch_core::AgentIdentity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "BindingInput")]
pub struct IdentityBinding {
    pub(crate) id: String,
    pub(crate) organization: String,
    pub(crate) team: String,
    pub(crate) owner: String,
    pub(crate) purpose: String,
    pub(crate) runtime: String,
    pub(crate) environment: String,
    pub(crate) device: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BindingInput {
    id: String,
    organization: String,
    team: String,
    owner: String,
    purpose: String,
    runtime: String,
    environment: String,
    device: String,
}

impl TryFrom<BindingInput> for IdentityBinding {
    type Error = ManifestError;
    fn try_from(value: BindingInput) -> Result<Self, Self::Error> {
        let binding = Self {
            id: value.id,
            organization: value.organization,
            team: value.team,
            owner: value.owner,
            purpose: value.purpose,
            runtime: value.runtime,
            environment: value.environment,
            device: value.device,
        };
        binding.validate()?;
        Ok(binding)
    }
}

impl IdentityBinding {
    pub fn from_identity(identity: &AgentIdentity) -> Result<Self, ManifestError> {
        Self::try_from(BindingInput {
            id: identity.id.clone(),
            organization: identity.organization.clone(),
            team: identity.team.clone(),
            owner: identity.owner.clone(),
            purpose: identity.purpose.clone(),
            runtime: identity.runtime.clone(),
            environment: identity.environment.clone(),
            device: identity.device.clone(),
        })
    }
}
