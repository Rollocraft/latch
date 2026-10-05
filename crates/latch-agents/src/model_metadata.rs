use crate::{ManifestError, error::validate_text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ModelInput")]
pub struct ModelMetadata {
    provider: String,
    model: String,
    version: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelInput {
    provider: String,
    model: String,
    version: String,
}

impl TryFrom<ModelInput> for ModelMetadata {
    type Error = ManifestError;
    fn try_from(value: ModelInput) -> Result<Self, Self::Error> {
        Self::new(value.provider, value.model, value.version)
    }
}

impl ModelMetadata {
    pub fn new(
        provider: impl Into<String>,
        model: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ManifestError> {
        let (provider, model, version) = (provider.into(), model.into(), version.into());
        validate_text("model.provider", &provider)?;
        validate_text("model.model", &model)?;
        validate_text("model.version", &version)?;
        Ok(Self {
            provider,
            model,
            version,
        })
    }
    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn model(&self) -> &str {
        &self.model
    }
    pub fn version(&self) -> &str {
        &self.version
    }
}
