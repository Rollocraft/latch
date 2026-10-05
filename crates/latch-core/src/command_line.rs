use super::CommandError;
use super::command_validation::{normalize_program, validate_arguments};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandLine {
    program: String,
    arguments: Vec<String>,
}

impl CommandLine {
    /// Directories and Windows executable suffixes are removed; names are lowercased.
    /// Thus a full executable path classifies like its bare program name.
    pub fn new(program: &str, arguments: Vec<String>) -> Result<Self, CommandError> {
        validate_arguments(&arguments)?;
        Ok(Self {
            program: normalize_program(program)?,
            arguments,
        })
    }
    pub fn program(&self) -> &str {
        &self.program
    }
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}
