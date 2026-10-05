use crate::NetworkError;
use std::{fmt, net::IpAddr};

/// A validated, normalized domain name: lowercase ASCII, no trailing dot.
/// Unicode must already be punycode, so that one name has one spelling and a
/// rule cannot be side-stepped by a different encoding of the same string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainName(String);

impl DomainName {
    pub fn new(value: &str) -> Result<Self, NetworkError> {
        let value = value.to_ascii_lowercase();
        let value = value.strip_suffix('.').unwrap_or(&value);
        if value.is_empty() {
            return Err(NetworkError::EmptyName);
        }
        if value.len() > 253 {
            return Err(NetworkError::NameTooLong);
        }
        if value.parse::<IpAddr>().is_ok() {
            return Err(NetworkError::AddressAsName);
        }
        let labels: Vec<&str> = value.split('.').collect();
        for label in &labels {
            if label.is_empty() || label.len() > 63 {
                return Err(NetworkError::InvalidLabel);
            }
            if label.starts_with('-') || label.ends_with('-') {
                return Err(NetworkError::InvalidLabel);
            }
            if !label
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            {
                return Err(NetworkError::InvalidLabel);
            }
        }
        // An all-numeric final label is how a malformed address sneaks in as a
        // name; no real top-level domain looks like that.
        if labels
            .last()
            .is_some_and(|last| last.bytes().all(|b| b.is_ascii_digit()))
        {
            return Err(NetworkError::AddressAsName);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.0.split('.')
    }

    pub(crate) fn is_subdomain_of(&self, parent: &Self) -> bool {
        self.0
            .strip_suffix(&parent.0)
            .and_then(|prefix| prefix.strip_suffix('.'))
            .is_some()
    }
}

impl fmt::Display for DomainName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
