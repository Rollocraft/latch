use crate::transaction_workspace::open;
use latch_transaction::Selection;
use std::{ffi::OsString, io::Write};

pub(crate) fn commit(
    workspace: &OsString,
    args: &[OsString],
    output: &mut impl Write,
) -> Result<(), String> {
    let mut transaction = open(workspace)?;
    let paths: Vec<&str> = args[2..]
        .iter()
        .map(|path| path.to_str().ok_or("paths must be UTF-8"))
        .collect::<Result<_, _>>()?;
    let selection = if paths.is_empty() {
        Selection::All
    } else {
        Selection::paths(paths).map_err(|e| format!("commit: invalid path: {e:?}"))?
    };
    let summary = transaction
        .commit(&selection)
        .map_err(|e| format!("commit: {e}"))?;
    writeln!(output, "committed\n{summary}").map_err(|e| e.to_string())
}
