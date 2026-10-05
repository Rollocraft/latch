use super::CommandLine;

impl CommandLine {
    pub fn has_flag(&self, flag: &str) -> bool {
        has_flag(self.arguments(), flag)
    }
    pub(super) fn has_any_flag(&self, flags: &[&str]) -> bool {
        flags.iter().any(|flag| self.has_flag(flag))
    }
}

/// Matches exact flags, `--flag=value`, and alphanumeric short-flag bundles.
/// A bare `--` ends options; `-o=file` is not a short-flag bundle.
pub fn has_flag(arguments: &[String], flag: &str) -> bool {
    let Some(name) = flag.strip_prefix('-') else {
        return false;
    };
    let long = name.starts_with('-');
    arguments
        .iter()
        .take_while(|argument| *argument != "--")
        .any(|argument| {
            if argument == flag {
                return true;
            }
            if long {
                return argument
                    .split_once('=')
                    .is_some_and(|(head, _)| head == flag);
            }
            let Some(letters) = argument.strip_prefix('-') else {
                return false;
            };
            !letters.starts_with('-')
                && !letters.is_empty()
                && letters.chars().all(|c| c.is_ascii_alphanumeric())
                && name.len() == 1
                && letters.contains(name)
        })
}
