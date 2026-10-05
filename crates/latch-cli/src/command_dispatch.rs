use crate::{
    audit_segment_output::audit,
    command_help::{CAPABILITIES, HELP},
    manifest_commands::manifest,
    policy_commands::policy,
    transaction_commands::transaction,
};
use std::{ffi::OsString, io::Write};

pub(crate) fn run(args: Vec<OsString>, output: &mut impl Write) -> Result<(), String> {
    if args.is_empty() || args == [OsString::from("--help")] || args == [OsString::from("-h")] {
        return output.write_all(HELP.as_bytes()).map_err(|e| e.to_string());
    }
    if args == [OsString::from("--version")] || args == [OsString::from("-V")] {
        return writeln!(output, "latch {}", env!("CARGO_PKG_VERSION")).map_err(|e| e.to_string());
    }
    match args[0].to_str() {
        Some("audit") if args.len() == 2 => audit(&args[1], output),
        Some("tx") if args.len() >= 2 => transaction(&args[1..], output),
        Some("status" | "diff" | "commit" | "rollback") => {
            let mut args = args;
            if args[0] == "diff" {
                args[0] = "changes".into();
            }
            transaction(&args, output)
        }
        Some("policy") => policy(&args[1..], output),
        Some("manifest") => manifest(&args[1..], output),
        Some("capabilities") if args.len() == 1 => output
            .write_all(CAPABILITIES.as_bytes())
            .map_err(|e| e.to_string()),
        Some("run") => Err("run is unsupported: no confined execution backend is connected; no process was started".into()),
        _ => Err("unsupported command or arguments; run latch --help".into()),
    }
}
