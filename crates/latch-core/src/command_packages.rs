use super::Classification;
use super::command_classification::classification;
use crate::Reversibility::*;

pub(super) fn classify_package(subcommand: &str) -> Classification {
    match subcommand {
        "publish" => classification("package.publish", Irreversible, 85),
        "install" | "ci" | "add" | "update" => {
            classification("package.install", PartiallyReversible, 40)
        }
        // A package script can do anything the shell can.
        "run" | "run-script" | "exec" | "dlx" => classification("package.script", Irreversible, 60),
        _ => classification("package.run", PartiallyReversible, 35),
    }
}
