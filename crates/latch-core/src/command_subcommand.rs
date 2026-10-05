use super::CommandLine;

impl CommandLine {
    /// Finds the first non-option argument, skipping known separate option values.
    /// A bare `--` ends this search; `--flag=value` consumes no next argument.
    pub fn subcommand(&self) -> Option<&str> {
        let options = value_options(self.program());
        let mut skip_value = false;
        for argument in self.arguments().iter().take_while(|a| *a != "--") {
            if skip_value {
                skip_value = false;
                continue;
            }
            if argument.starts_with('-') {
                skip_value = options.contains(&argument.as_str());
                continue;
            }
            return Some(argument);
        }
        None
    }
}

/// Only options whose value is a separate argument need to be listed.
fn value_options(program: &str) -> &'static [&'static str] {
    match program {
        "git" => &[
            "-C",
            "-c",
            "--git-dir",
            "--work-tree",
            "--exec-path",
            "--namespace",
        ],
        "docker" | "podman" | "nerdctl" => {
            &["-H", "--host", "--context", "--config", "--log-level"]
        }
        "kubectl" | "helm" => &[
            "-n",
            "--namespace",
            "--context",
            "--kubeconfig",
            "--cluster",
            "--user",
            "--as",
        ],
        "npm" | "pnpm" | "yarn" | "bun" => &["--prefix", "--registry", "-w", "--workspace"],
        _ => &[],
    }
}
