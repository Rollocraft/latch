use crate::ManifestError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ResourceId(String);

impl ResourceId {
    pub fn new(value: impl Into<String>) -> Result<Self, ManifestError> {
        let value = value.into();
        crate::resource_uri_validation::validate_resource(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn scheme(&self) -> &str {
        self.0.split_once("://").expect("validated resource URI").0
    }

    pub(crate) fn within(&self, root: &Self) -> bool {
        self == root
            || self
                .0
                .strip_prefix(&root.0)
                .is_some_and(|tail| tail.starts_with('/'))
    }
}

impl TryFrom<String> for ResourceId {
    type Error = ManifestError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ResourceId> for String {
    fn from(value: ResourceId) -> Self {
        value.0
    }
}
