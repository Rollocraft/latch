use crate::{ManifestError, ModelMetadata, error::validate_label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "AgentInput")]
pub struct AgentDescriptor {
    name: String,
    version: String,
    model_metadata: Option<ModelMetadata>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentInput {
    name: String,
    version: String,
    model_metadata: Option<ModelMetadata>,
}

impl TryFrom<AgentInput> for AgentDescriptor {
    type Error = ManifestError;
    fn try_from(value: AgentInput) -> Result<Self, Self::Error> {
        Self::new(value.name, value.version, value.model_metadata)
    }
}

impl AgentDescriptor {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        model_metadata: Option<ModelMetadata>,
    ) -> Result<Self, ManifestError> {
        let (name, version) = (name.into(), version.into());
        validate_label("agent.name", &name)?;
        validate_label("agent.version", &version)?;
        Ok(Self {
            name,
            version,
            model_metadata,
        })
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn model_metadata(&self) -> Option<&ModelMetadata> {
        self.model_metadata.as_ref()
    }
}
