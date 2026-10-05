use std::ffi::OsString;
use std::process::ExitCode;

fn response(arguments: &[OsString]) -> Result<&'static str, &'static str> {
    if arguments.is_empty()
        || (arguments.len() == 1 && (arguments[0] == "--help" || arguments[0] == "-h"))
    {
        Ok(usage::SERVICE_HELP)
    } else {
        Err(usage::NOT_CONFIGURED)
    }
}

mod usage {
    pub const SERVICE_HELP: &str = "latch-warden\nUsage: latch-warden [--help]\nService backend not configured; supervision commands unavailable.\nPersistent state, an actual enforcement backend, authorization and audit must be supplied externally.";
    pub const NOT_CONFIGURED: &str = "service backend not configured; commands unavailable";
}

fn main() -> ExitCode {
    match response(&std::env::args_os().skip(1).collect::<Vec<_>>()) {
        Ok(help) => {
            println!("{help}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("latch-warden: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_exposes_missing_service_configuration() {
        for arguments in [vec![], vec!["--help".into()], vec!["-h".into()]] {
            let help = response(&arguments).unwrap();
            assert!(help.contains("Service backend not configured"));
            assert!(help.contains("authorization and audit"));
        }
    }

    #[test]
    fn every_command_is_unavailable() {
        for command in ["start", "freeze", "terminate", "kill", "serve", "unknown"] {
            assert!(response(&[command.into()]).is_err());
            assert!(response(&[command.into(), "--help".into()]).is_err());
        }
        assert!(response(&["--help".into(), "start".into()]).is_err());
    }
}
