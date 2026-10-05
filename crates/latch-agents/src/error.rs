use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    InvalidField {
        field: &'static str,
        reason: &'static str,
    },
    UnsupportedSchemaVersion(u32),
    DuplicateCapability,
    TooManyCapabilities,
    ManifestTooLarge,
    IdentityMismatch,
    DifferentAgent,
    Core(latch_core::ValidationError),
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField { field, reason } => write!(f, "invalid {field}: {reason}"),
            Self::UnsupportedSchemaVersion(version) => {
                write!(f, "unsupported manifest schema version: {version}")
            }
            Self::DuplicateCapability => f.write_str("duplicate requested capability"),
            Self::TooManyCapabilities => f.write_str("too many requested capabilities"),
            Self::ManifestTooLarge => f.write_str("manifest exceeds the size limit"),
            Self::IdentityMismatch => {
                f.write_str("manifest identity binding does not match runtime attribution")
            }
            Self::DifferentAgent => f.write_str(
                "permission baselines must belong to the same agent and identity binding",
            ),
            Self::Core(error) => write!(f, "invalid runtime attribution: {error:?}"),
        }
    }
}

impl std::error::Error for ManifestError {}

impl From<latch_core::ValidationError> for ManifestError {
    fn from(value: latch_core::ValidationError) -> Self {
        Self::Core(value)
    }
}

pub(crate) fn invalid(field: &'static str, reason: &'static str) -> ManifestError {
    ManifestError::InvalidField { field, reason }
}

pub(crate) fn validate_text(field: &'static str, text: &str) -> Result<(), ManifestError> {
    if text.is_empty()
        || text.len() > 512
        || text.trim() != text
        || text.chars().any(char::is_control)
    {
        return Err(invalid(
            field,
            "expected 1..=512 bytes without surrounding whitespace or controls",
        ));
    }
    Ok(())
}

pub(crate) fn validate_label(field: &'static str, text: &str) -> Result<(), ManifestError> {
    validate_text(field, text)?;
    if !text
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
        || text == "."
        || text == ".."
    {
        return Err(invalid(field, "expected an exact ASCII identifier"));
    }
    Ok(())
}
