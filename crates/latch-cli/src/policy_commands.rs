use crate::{
    bounded_file_read::bounded_read, policy_document_read::load_policy, policy_explain, scenarios,
};
use latch_policy::MAX_POLICY_DOCUMENT_BYTES;
use std::{ffi::OsString, io::Write};

pub(crate) fn policy(args: &[OsString], output: &mut impl Write) -> Result<(), String> {
    match args.first().and_then(|arg| arg.to_str()) {
        Some("explain") => policy_explain::explain(&args[1..], output),
        Some("validate") if args.len() == 2 => {
            let document = load_policy(&args[1])?;
            writeln!(
                output,
                "valid policy document v{}: {} policies",
                document.version(),
                document.policies().len()
            )
            .map_err(|e| e.to_string())
        }
        Some("test") if args.len() == 2 && args[1] == "--help" => output
            .write_all(scenarios::HELP.as_bytes())
            .map_err(|e| e.to_string()),
        Some("test") if args.len() == 3 => {
            let engine = load_policy(&args[1])?.into_engine();
            let bytes = bounded_read(&args[2], MAX_POLICY_DOCUMENT_BYTES)?;
            scenarios::test(&engine, &bytes, output)
        }
        _ => Err("unsupported policy command or arguments; run latch --help".into()),
    }
}
