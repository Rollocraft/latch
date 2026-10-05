use crate::{DomainName, NetworkError};
use std::fmt;

/// `*` matches every name. `*.example.com` matches names *below*
/// example.com but not example.com itself, so that a wildcard grants no more
/// than it says and the parent falls through to whatever else applies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Any,
    Exact(DomainName),
    Subdomains(DomainName),
}

impl Pattern {
    pub fn parse(value: &str) -> Result<Self, NetworkError> {
        let value = value.trim().to_ascii_lowercase();
        if value == "*" {
            return Ok(Self::Any);
        }
        match value.strip_prefix("*.") {
            Some(rest) => Ok(Self::Subdomains(DomainName::new(rest)?)),
            None if value.contains('*') => Err(NetworkError::InvalidPattern),
            None => Ok(Self::Exact(DomainName::new(&value)?)),
        }
    }

    pub fn matches(&self, name: &DomainName) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(pattern) => pattern == name,
            Self::Subdomains(parent) => name.is_subdomain_of(parent),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str("*"),
            Self::Exact(name) => write!(f, "{name}"),
            Self::Subdomains(name) => write!(f, "*.{name}"),
        }
    }
}
