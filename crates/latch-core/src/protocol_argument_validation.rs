use super::{MAX_ARGUMENT_BYTES, MAX_ARGUMENTS, MAX_TOTAL_ARGUMENT_BYTES, ProtocolError};

pub(super) fn validate_arguments(arguments: &[String]) -> Result<(), ProtocolError> {
    if arguments.len() > MAX_ARGUMENTS {
        return Err(ProtocolError::LimitExceeded { field: "arguments" });
    }
    let mut total = 0usize;
    for argument in arguments {
        if argument.len() > MAX_ARGUMENT_BYTES {
            return Err(ProtocolError::LimitExceeded { field: "argument" });
        }
        if argument.contains('\0') {
            return Err(ProtocolError::InvalidField { field: "argument" });
        }
        total += argument.len();
    }
    if total > MAX_TOTAL_ARGUMENT_BYTES {
        return Err(ProtocolError::LimitExceeded { field: "arguments" });
    }
    Ok(())
}
