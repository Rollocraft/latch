use crate::PolicyError;
use std::fmt;

pub const POLICY_DOCUMENT_VERSION: u64 = 1;
pub const MAX_POLICY_DOCUMENT_BYTES: usize = 1024 * 1024;
pub const MAX_POLICY_DOCUMENT_DEPTH: usize = 32;
pub const MAX_POLICY_DOCUMENT_NODES: usize = 32_768;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyFormat {
    Json,
    Yaml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyLoadError {
    TooLarge { actual: usize, maximum: usize },
    InvalidDocument(String),
    UnsupportedVersion(u64),
    InvalidPolicy(PolicyError),
}

impl fmt::Display for PolicyLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge { actual, maximum } => {
                write!(f, "policy document is {actual} bytes; maximum is {maximum}")
            }
            Self::InvalidDocument(message) => write!(f, "invalid policy document: {message}"),
            Self::UnsupportedVersion(version) => write!(f, "unsupported policy version: {version}"),
            Self::InvalidPolicy(error) => write!(f, "invalid policy: {error:?}"),
        }
    }
}

impl std::error::Error for PolicyLoadError {}
