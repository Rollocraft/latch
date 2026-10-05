use crate::transaction_workspace::open;
use latch_transaction::Change;
use std::{ffi::OsString, io::Write};

pub(crate) fn show_changes(workspace: &OsString, output: &mut impl Write) -> Result<(), String> {
    let transaction = open(workspace)?;
    let changes = transaction.changes().map_err(|e| format!("changes: {e}"))?;
    for change in changes.changes() {
        let line = match change {
            Change::Created { path, .. } => format!("new      {:?}", path.as_str()),
            Change::Modified {
                path, permissions, ..
            } => format!(
                "{}  {:?}",
                if *permissions { "mode    " } else { "modified" },
                path.as_str()
            ),
            Change::Deleted { path, .. } => format!("deleted  {:?}", path.as_str()),
            Change::Renamed { from, to, .. } => {
                format!("renamed  {:?} -> {:?}", from.as_str(), to.as_str())
            }
        };
        writeln!(output, "{line}").map_err(|e| e.to_string())?;
    }
    writeln!(output, "\n{}", changes.summary()).map_err(|e| e.to_string())
}
