//! Shell commands as semantic actions.
//!
//! A command is a program plus separate arguments, never an implicitly parsed
//! shell string. Classification is advisory, not a security boundary: a program
//! name can be borrowed. Shells are high-risk actions rather than wrappers.

/// Bounds what an agent can hand over in one call.
pub const MAXIMUM_ARGUMENTS: usize = 4096;
pub const MAXIMUM_ARGUMENT_BYTES: usize = 128 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    EmptyProgram,
    IllegalCharacter,
    TooManyArguments,
    ArgumentTooLong,
}

#[path = "command_classification.rs"]
mod command_classification;
#[path = "command_flags.rs"]
mod command_flags;
#[path = "command_git.rs"]
mod command_git;
#[path = "command_infrastructure.rs"]
mod command_infrastructure;
#[path = "command_line.rs"]
mod command_line;
#[path = "command_packages.rs"]
mod command_packages;
#[path = "command_subcommand.rs"]
mod command_subcommand;
#[path = "command_validation.rs"]
mod command_validation;

pub use command_classification::Classification;
pub use command_flags::has_flag;
pub use command_line::CommandLine;

#[cfg(test)]
use crate::Reversibility;
#[cfg(test)]
#[path = "command_classification_tests.rs"]
mod command_classification_tests;
#[cfg(test)]
#[path = "command_flags_tests.rs"]
mod command_flags_tests;
#[cfg(test)]
#[path = "command_normalization_tests.rs"]
mod command_normalization_tests;
#[cfg(test)]
#[path = "command_shell_classification_tests.rs"]
mod command_shell_classification_tests;
#[cfg(test)]
#[path = "command_subcommands_tests.rs"]
mod command_subcommands_tests;
#[cfg(test)]
#[path = "command_test_support.rs"]
mod command_test_support;
