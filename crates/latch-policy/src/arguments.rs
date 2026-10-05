//! Argument-aware matching.
//!
//! `git status` and `git push --force origin main` are the same program, and a
//! rule that can only name the program cannot tell them apart. These matchers
//! let a rule say what it actually means, on the parsed argument vector rather
//! than on a command string that an agent chooses the spelling of.
//!
//! Flags are matched in the forms a command line really uses, so a rule
//! written for `--force` is not defeated by `-f`, `-rf` or `--force=true`.
//! What no matcher can see through is a shell: `latch-core` classifies that as
//! its own high-risk action for exactly this reason.

use latch_core::has_flag;

/// How deeply `All` may nest. A policy is data, sometimes from a file, so the
/// bound exists before evaluation rather than as a hope about its authors.
pub const MAXIMUM_DEPTH: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ArgumentMatcher {
    /// Any arguments at all, including none. The default for a rule that is
    /// about the action rather than how it was invoked.
    #[default]
    Any,
    /// No arguments.
    Empty,
    /// The complete vector, in order. The strictest form, and the one to reach
    /// for when a rule is meant to permit exactly one invocation.
    Exactly(Vec<String>),
    /// This flag is present, written as it appears on a command line.
    Flag(String),
    /// Any one of these flags is present.
    AnyFlag(Vec<String>),
    /// Some argument equals this string.
    Contains(String),
    /// The argument at this index equals this value.
    Positional { index: usize, value: String },
    /// Every one of these matches.
    All(Vec<ArgumentMatcher>),
}

impl ArgumentMatcher {
    pub fn matches(&self, arguments: &[String]) -> bool {
        match self {
            Self::Any => true,
            Self::Empty => arguments.is_empty(),
            Self::Exactly(expected) => expected == arguments,
            Self::Flag(flag) => has_flag(arguments, flag),
            Self::AnyFlag(flags) => flags.iter().any(|flag| has_flag(arguments, flag)),
            Self::Contains(value) => arguments.contains(value),
            Self::Positional { index, value } => arguments.get(*index) == Some(value),
            Self::All(matchers) => matchers.iter().all(|matcher| matcher.matches(arguments)),
        }
    }

    /// Reject a matcher that cannot mean anything: an empty flag, a flag not
    /// written as one, a control character, or nesting past the depth bound.
    /// An `All` with nothing in it would match everything, which is what `Any`
    /// says out loud.
    pub fn valid(&self) -> bool {
        self.valid_to_depth(MAXIMUM_DEPTH)
    }

    fn valid_to_depth(&self, remaining: usize) -> bool {
        let text = |value: &String| !value.is_empty() && !value.chars().any(char::is_control);
        let flag = |value: &String| {
            text(value)
                && value.starts_with('-')
                && value.len() > 1
                && value != "--"
                && !value.chars().any(char::is_whitespace)
        };
        match self {
            Self::Any | Self::Empty => true,
            Self::Exactly(values) => values.iter().all(text),
            Self::Flag(value) => flag(value),
            Self::AnyFlag(values) => !values.is_empty() && values.iter().all(flag),
            Self::Contains(value) => text(value),
            Self::Positional { value, .. } => text(value),
            Self::All(matchers) => {
                remaining > 0
                    && !matchers.is_empty()
                    && matchers
                        .iter()
                        .all(|matcher| matcher.valid_to_depth(remaining - 1))
            }
        }
    }
}

#[cfg(test)]
#[path = "argument_matching_tests.rs"]
mod argument_matching_tests;
