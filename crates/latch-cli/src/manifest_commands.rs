use crate::bounded_file_read::bounded_read;
use latch_agents::{AgentManifest, MAX_MANIFEST_BYTES, PermissionDiff};
use std::{ffi::OsString, io::Write};

fn load_manifest(path: &OsString) -> Result<AgentManifest, String> {
    let bytes = bounded_read(path, MAX_MANIFEST_BYTES)?;
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("manifest must be UTF-8: {e}"))?;
    AgentManifest::from_json(text).map_err(|e| format!("invalid manifest {path:?}: {e:?}"))
}

pub(crate) fn manifest(args: &[OsString], output: &mut impl Write) -> Result<(), String> {
    match args.first().and_then(|arg| arg.to_str()) {
        Some("check") if args.len() == 2 => {
            let manifest = load_manifest(&args[1])?;
            writeln!(
                output,
                "valid manifest v{}: {:?}, {} requested capabilities; not authorization",
                manifest.schema_version(),
                manifest.agent().name(),
                manifest.requires().len()
            )
            .map_err(|e| e.to_string())
        }
        Some("diff") if args.len() == 3 => {
            let old = load_manifest(&args[1])?;
            let new = load_manifest(&args[2])?;
            let diff =
                PermissionDiff::between(&old, &new).map_err(|e| format!("manifest diff: {e:?}"))?;
            serde_json::to_writer_pretty(&mut *output, &diff).map_err(|e| e.to_string())?;
            writeln!(
                output,
                "\nrequires_review: {}; requested permissions only, not grants",
                diff.requires_review()
            )
            .map_err(|e| e.to_string())
        }
        _ => Err("unsupported manifest command or arguments; run latch --help".into()),
    }
}
