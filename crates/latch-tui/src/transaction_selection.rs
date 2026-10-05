use latch_transaction::{Change, ChangeSet};

use crate::{MAX_ROWS, Output, Rendered};

/// Public entry: renders a change set with explicit zero-based selections.
pub fn render_transaction_selection(changes: &ChangeSet, selected: &[usize]) -> Rendered {
    render_changes(changes.changes(), selected)
}

/// Metadata-only change review; contents and digests are never displayed.
pub(crate) fn render_changes(changes: &[Change], selected: &[usize]) -> Rendered {
    let mut out = Output::default();
    out.line("TRANSACTION REVIEW - zero-based selection; no commit or rollback performed");
    out.line("Selections apply only to this supplied change set; contents and digests hidden.");
    if changes.is_empty() {
        out.line("No changes");
    }
    if selected.len() > MAX_ROWS {
        out.line("[INVALID SELECTION] Too many selections; no selection displayed");
    } else if selected.iter().any(|index| *index >= changes.len()) {
        out.line("[INVALID SELECTION] Unknown index; no selection displayed");
    }
    let valid = selected.len() <= MAX_ROWS && selected.iter().all(|index| *index < changes.len());
    for (index, change) in changes.iter().take(MAX_ROWS).enumerate() {
        let checked = if valid && selected.contains(&index) {
            "x"
        } else {
            " "
        };
        let kind = match change {
            Change::Created { .. } => "CREATED",
            Change::Modified {
                permissions: true, ..
            } => "MODIFIED (permissions changed)",
            Change::Modified { .. } => "MODIFIED",
            Change::Deleted { .. } => "DELETED",
            Change::Renamed { .. } => "RENAMED",
        };
        out.line(&format!("[{checked}] {index}: {kind}"));
        if let Change::Renamed { from, .. } = change {
            out.field("  From", from.as_str());
        }
        out.field("  Path", change.path().as_str());
    }
    out.omitted(changes.len());
    out.finish()
}
