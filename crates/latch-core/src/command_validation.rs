use super::{CommandError, MAXIMUM_ARGUMENT_BYTES, MAXIMUM_ARGUMENTS};

pub(super) fn validate_arguments(arguments: &[String]) -> Result<(), CommandError> {
    if arguments.len() > MAXIMUM_ARGUMENTS {
        return Err(CommandError::TooManyArguments);
    }
    for argument in arguments {
        if argument.len() > MAXIMUM_ARGUMENT_BYTES {
            return Err(CommandError::ArgumentTooLong);
        }
        if argument.contains('\0') {
            return Err(CommandError::IllegalCharacter);
        }
    }
    Ok(())
}

pub(super) fn normalize_program(program: &str) -> Result<String, CommandError> {
    let name = program
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(program)
        .to_ascii_lowercase();
    let name = [".exe", ".cmd", ".bat", ".com", ".ps1"]
        .iter()
        .find_map(|suffix| name.strip_suffix(suffix))
        .unwrap_or(&name)
        .to_owned();
    if name.trim().is_empty() {
        return Err(CommandError::EmptyProgram);
    }
    if name.chars().any(char::is_control) {
        return Err(CommandError::IllegalCharacter);
    }
    Ok(name)
}
