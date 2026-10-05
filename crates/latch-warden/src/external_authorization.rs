use std::error::Error;
use std::fmt;

/// Organization scoping handed over by an already-authenticated control plane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalAuthorization {
    organization: String,
}

impl ExternalAuthorization {
    pub fn from_trusted_control_plane(
        organization: impl Into<String>,
    ) -> Result<Self, InvalidIdentifier> {
        let organization = organization.into();
        validate_identifier(&organization)?;
        Ok(Self { organization })
    }

    pub fn organization(&self) -> &str {
        &self.organization
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidIdentifier;

impl fmt::Display for InvalidIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "identifier must contain 1..=256 bytes without whitespace or control characters",
        )
    }
}

impl Error for InvalidIdentifier {}

/// Shared identifier rule for organization and session identifiers.
pub(crate) fn validate_identifier(value: &str) -> Result<(), InvalidIdentifier> {
    if value.is_empty()
        || value.len() > 256
        || value.chars().any(|c| c.is_whitespace() || c.is_control())
    {
        Err(InvalidIdentifier)
    } else {
        Ok(())
    }
}
