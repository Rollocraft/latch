use super::{MAX_MESSAGE_BYTES, ProtocolError, VERSION};

pub(super) fn version(received: u32) -> Result<(), ProtocolError> {
    if received != VERSION {
        return Err(ProtocolError::UnsupportedVersion { received });
    }
    Ok(())
}

pub(super) fn text(value: &str, maximum: usize, field: &'static str) -> Result<(), ProtocolError> {
    if value.len() > maximum {
        return Err(ProtocolError::LimitExceeded { field });
    }
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(ProtocolError::InvalidField { field });
    }
    Ok(())
}

pub(super) fn bounded(bytes: &[u8]) -> Result<(), ProtocolError> {
    if bytes.len() > MAX_MESSAGE_BYTES {
        return Err(ProtocolError::MessageTooLarge);
    }
    Ok(())
}
