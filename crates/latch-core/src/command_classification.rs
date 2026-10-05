use super::CommandLine;
use super::command_git::classify_git;
use super::command_infrastructure::{
    classify_container, classify_infrastructure, classify_kubernetes,
};
use super::command_packages::classify_package;
use crate::{Reversibility, RiskScore};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    pub action: String,
    pub reversibility: Reversibility,
    pub risk: RiskScore,
}

pub(super) fn classification(
    action: &str,
    reversibility: Reversibility,
    risk: u8,
) -> Classification {
    Classification {
        action: action.to_owned(),
        reversibility,
        risk: RiskScore::new(risk).expect("classification risk within range"),
    }
}

impl CommandLine {
    /// Advisory default classification, not a security boundary: names can be borrowed.
    /// Unknown programs are irreversible; shells are not treated as transparent wrappers.
    pub fn classify(&self) -> Classification {
        use Reversibility::*;
        let subcommand = self.subcommand().unwrap_or("");
        match self.program() {
            "sh" | "bash" | "zsh" | "dash" | "fish" | "ksh" | "cmd" | "powershell" | "pwsh" => {
                classification("command.shell", Irreversible, 90)
            }
            "sudo" | "su" | "doas" | "runas" => {
                classification("command.privilege_escalation", Irreversible, 95)
            }
            "rm" | "rmdir" | "del" => {
                if self.has_any_flag(&["-r", "-R", "--recursive"]) {
                    classification("file.recursive_delete", Irreversible, 80)
                } else {
                    classification("file.delete", PartiallyReversible, 45)
                }
            }
            "git" => classify_git(self, subcommand),
            "npm" | "pnpm" | "yarn" | "bun" => classify_package(subcommand),
            "docker" | "podman" | "nerdctl" => classify_container(subcommand),
            "terraform" | "tofu" => classify_infrastructure(subcommand),
            "kubectl" | "helm" => classify_kubernetes(subcommand),
            "curl" | "wget" | "nc" | "ncat" => {
                classification("network.request", PartiallyReversible, 45)
            }
            "cat" | "ls" | "head" | "tail" | "grep" | "find" | "wc" | "dir" => {
                classification("file.read", FullyReversible, 5)
            }
            _ => classification("command.run", Irreversible, 50),
        }
    }
}
