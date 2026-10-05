use super::{bounded_document_value::BoundedValue, wire_policy::WireDocument};
use crate::{
    MAX_POLICY_DOCUMENT_BYTES, MAX_POLICY_DOCUMENT_NODES, POLICY_DOCUMENT_VERSION, Policy,
    PolicyDocument, PolicyEngine, PolicyFormat, PolicyLoadError,
};
use serde::de::DeserializeSeed;
use std::fmt;

pub fn load_policy_document(
    input: &[u8],
    format: PolicyFormat,
) -> Result<PolicyDocument, PolicyLoadError> {
    if input.len() > MAX_POLICY_DOCUMENT_BYTES {
        return Err(PolicyLoadError::TooLarge {
            actual: input.len(),
            maximum: MAX_POLICY_DOCUMENT_BYTES,
        });
    }
    let mut remaining = MAX_POLICY_DOCUMENT_NODES;
    let mut bytes = MAX_POLICY_DOCUMENT_BYTES;
    let seed = BoundedValue {
        depth: 0,
        remaining: &mut remaining,
        bytes: &mut bytes,
    };
    let value = match format {
        PolicyFormat::Json => {
            let mut deserializer = serde_json::Deserializer::from_slice(input);
            let value = seed
                .deserialize(&mut deserializer)
                .map_err(invalid_document)?;
            deserializer.end().map_err(invalid_document)?;
            value
        }
        PolicyFormat::Yaml => {
            let mut documents = serde_yaml::Deserializer::from_slice(input);
            let document = documents
                .next()
                .ok_or_else(|| PolicyLoadError::InvalidDocument("empty YAML document".into()))?;
            let value = seed.deserialize(document).map_err(invalid_document)?;
            if documents.next().is_some() {
                return Err(PolicyLoadError::InvalidDocument(
                    "expected exactly one YAML document".into(),
                ));
            }
            value
        }
    };
    let document: WireDocument = serde_json::from_value(value).map_err(invalid_document)?;
    if document.version != POLICY_DOCUMENT_VERSION {
        return Err(PolicyLoadError::UnsupportedVersion(document.version));
    }
    let policies = document.policies.into_iter().map(Policy::from).collect();
    let engine = PolicyEngine::new(policies).map_err(PolicyLoadError::InvalidPolicy)?;
    Ok(PolicyDocument {
        policies: engine.policies,
    })
}

fn invalid_document(error: impl fmt::Display) -> PolicyLoadError {
    PolicyLoadError::InvalidDocument(error.to_string())
}
