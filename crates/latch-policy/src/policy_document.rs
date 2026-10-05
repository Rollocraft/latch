use crate::{
    POLICY_DOCUMENT_VERSION, Policy, PolicyEngine, PolicyFormat, PolicyLoadError,
    load_policy_document,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDocument {
    pub(super) policies: Vec<Policy>,
}

impl PolicyDocument {
    pub fn from_json(input: &[u8]) -> Result<Self, PolicyLoadError> {
        load_policy_document(input, PolicyFormat::Json)
    }

    pub fn from_yaml(input: &[u8]) -> Result<Self, PolicyLoadError> {
        load_policy_document(input, PolicyFormat::Yaml)
    }

    pub fn version(&self) -> u64 {
        POLICY_DOCUMENT_VERSION
    }

    pub fn policies(&self) -> &[Policy] {
        &self.policies
    }

    pub fn into_policies(self) -> Vec<Policy> {
        self.policies
    }

    pub fn into_engine(self) -> PolicyEngine {
        PolicyEngine {
            policies: self.policies,
        }
    }
}
