use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    InvalidJson,
    UnsupportedVersion { received: u32 },
    MessageTooLarge,
    InvalidField { field: &'static str },
    LimitExceeded { field: &'static str },
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson => f.write_str("invalid control JSON"),
            Self::UnsupportedVersion { .. } => f.write_str("unsupported control version"),
            Self::MessageTooLarge => f.write_str("control message exceeds limit"),
            Self::InvalidField { field } => write!(f, "invalid field: {field}"),
            Self::LimitExceeded { field } => write!(f, "field exceeds limit: {field}"),
        }
    }
}
impl std::error::Error for ProtocolError {}
