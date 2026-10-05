use latch_core::AgentIdentity;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IdentityInput {
    id: String,
    organization: String,
    team: String,
    owner: String,
    purpose: String,
    provider: String,
    model: String,
    model_version: String,
    runtime: String,
    environment: String,
    device: String,
    created_at: u64,
    expires_at: u64,
    trust_level: u8,
}

impl From<IdentityInput> for AgentIdentity {
    fn from(input: IdentityInput) -> Self {
        Self {
            id: input.id,
            organization: input.organization,
            team: input.team,
            owner: input.owner,
            purpose: input.purpose,
            provider: input.provider,
            model: input.model,
            model_version: input.model_version,
            runtime: input.runtime,
            environment: input.environment,
            device: input.device,
            created_at: input.created_at,
            expires_at: input.expires_at,
            trust_level: input.trust_level,
        }
    }
}
