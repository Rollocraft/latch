use super::Classification;
use super::command_classification::classification;
use crate::Reversibility::*;

pub(super) fn classify_container(subcommand: &str) -> Classification {
    match subcommand {
        "push" => classification("container.publish", Irreversible, 80),
        "rm" | "rmi" | "prune" | "down" | "kill" => {
            classification("container.remove", PartiallyReversible, 60)
        }
        "ps" | "logs" | "inspect" | "images" => {
            classification("container.inspect", FullyReversible, 5)
        }
        _ => classification("container.run", PartiallyReversible, 50),
    }
}

pub(super) fn classify_infrastructure(subcommand: &str) -> Classification {
    match subcommand {
        "destroy" => classification("infrastructure.destroy", Irreversible, 98),
        "apply" => classification("infrastructure.apply", PartiallyReversible, 90),
        "plan" | "validate" | "show" | "fmt" => {
            classification("infrastructure.plan", FullyReversible, 5)
        }
        _ => classification("infrastructure.run", PartiallyReversible, 60),
    }
}

pub(super) fn classify_kubernetes(subcommand: &str) -> Classification {
    match subcommand {
        "delete" | "uninstall" => classification("kubernetes.delete", Irreversible, 80),
        "apply" | "create" | "install" | "upgrade" | "patch" | "scale" => {
            classification("kubernetes.apply", PartiallyReversible, 70)
        }
        "get" | "describe" | "logs" | "explain" => {
            classification("kubernetes.inspect", FullyReversible, 5)
        }
        _ => classification("kubernetes.run", PartiallyReversible, 50),
    }
}
