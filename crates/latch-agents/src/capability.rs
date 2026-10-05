use crate::{ActionName, ManifestError, ResourceId, ResourceScope, error::validate_label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "CapabilityInput")]
pub struct RequestedCapability {
    action: ActionName,
    resource: ResourceScope,
    environment: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilityInput {
    action: ActionName,
    resource: ResourceScope,
    environment: String,
}

impl TryFrom<CapabilityInput> for RequestedCapability {
    type Error = ManifestError;
    fn try_from(value: CapabilityInput) -> Result<Self, Self::Error> {
        Self::new(value.action, value.resource, value.environment)
    }
}

impl RequestedCapability {
    pub fn new(
        action: ActionName,
        resource: ResourceScope,
        environment: impl Into<String>,
    ) -> Result<Self, ManifestError> {
        let environment = environment.into();
        validate_label("environment", &environment)?;
        Ok(Self {
            action,
            resource,
            environment,
        })
    }

    pub fn action(&self) -> &ActionName {
        &self.action
    }
    pub fn resource(&self) -> &ResourceScope {
        &self.resource
    }
    pub fn environment(&self) -> &str {
        &self.environment
    }

    pub fn covers(&self, other: &Self) -> bool {
        self.action == other.action
            && self.environment == other.environment
            && self.resource.covers(&other.resource)
    }

    pub fn requests(&self, action: &ActionName, resource: &ResourceId, environment: &str) -> bool {
        self.action == *action
            && self.environment == environment
            && self.resource.contains(resource)
    }
}
