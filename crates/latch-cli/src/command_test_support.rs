use crate::run;
use std::{ffi::OsString, path::PathBuf};

pub(crate) fn temporary(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "latch-cli-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

pub(crate) fn invoke(args: &[&OsString]) -> Result<String, String> {
    let mut output = Vec::new();
    run(args.iter().map(|a| (*a).clone()).collect(), &mut output)?;
    Ok(String::from_utf8(output).unwrap())
}
