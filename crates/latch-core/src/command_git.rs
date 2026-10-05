use super::command_classification::classification;
use super::{Classification, CommandLine};
use crate::Reversibility::*;

pub(super) fn classify_git(command: &CommandLine, subcommand: &str) -> Classification {
    match subcommand {
        "push" if command.has_any_flag(&["-f", "--force", "--force-with-lease"]) => {
            classification("git.force_push", Irreversible, 85)
        }
        "push" => classification("git.push", PartiallyReversible, 55),
        "commit" => classification("git.commit", FullyReversible, 15),
        "status" | "log" | "diff" | "show" | "blame" => {
            classification("git.inspect", FullyReversible, 2)
        }
        "clone" | "fetch" | "pull" => classification("git.fetch", FullyReversible, 10),
        "reset" if command.has_flag("--hard") => {
            classification("git.reset_hard", PartiallyReversible, 60)
        }
        "checkout" | "switch" | "branch" | "merge" | "rebase" | "reset" | "add" => {
            classification("git.modify", FullyReversible, 25)
        }
        _ => classification("git.run", PartiallyReversible, 40),
    }
}
