use super::ValidationError;
use super::identity_text::valid_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentIdentity {
    pub id: String,
    pub organization: String,
    pub team: String,
    pub owner: String,
    pub purpose: String,
    pub provider: String,
    pub model: String,
    pub model_version: String,
    pub runtime: String,
    pub environment: String,
    pub device: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub trust_level: u8,
}

impl AgentIdentity {
    pub fn validate_at(&self, now: u64) -> Result<(), ValidationError> {
        self.validate_uri()?;
        self.validate_fields()?;
        if self.trust_level > 100 {
            return Err(ValidationError::InvalidTrust);
        }
        if now < self.created_at || now >= self.expires_at {
            return Err(ValidationError::ExpiredIdentity);
        }
        Ok(())
    }

    fn validate_uri(&self) -> Result<(), ValidationError> {
        let path = self
            .id
            .strip_prefix("agent://")
            .ok_or(ValidationError::InvalidIdentity)?;
        if path.split('/').count() < 2
            || path
                .split('/')
                .any(|p| p.is_empty() || p == "." || p == "..")
            || !path
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/-_.".contains(&b))
            || path.split('/').next() != Some(self.organization.as_str())
        {
            return Err(ValidationError::InvalidIdentity);
        }
        Ok(())
    }

    fn validate_fields(&self) -> Result<(), ValidationError> {
        if [
            &self.organization,
            &self.team,
            &self.owner,
            &self.purpose,
            &self.provider,
            &self.model,
            &self.model_version,
            &self.runtime,
            &self.environment,
            &self.device,
        ]
        .iter()
        .any(|s| !valid_text(s))
        {
            return Err(ValidationError::MissingField);
        }
        Ok(())
    }
}
