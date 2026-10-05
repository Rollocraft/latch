use latch_transaction::Transaction;
use std::ffi::OsString;

pub(crate) fn open(workspace: &OsString) -> Result<Transaction, String> {
    Transaction::open(workspace).map_err(|e| {
        format!(
            "cannot open workspace {:?}: {e}",
            workspace.to_string_lossy()
        )
    })
}
