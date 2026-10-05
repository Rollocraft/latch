use crate::{transaction_status, transaction_workspace::open};
use latch_transaction::Transaction;
use std::{ffi::OsString, io::Write};

pub(crate) fn transaction(args: &[OsString], output: &mut impl Write) -> Result<(), String> {
    let workspace = args.get(1).ok_or("missing workspace path")?;
    match args[0].to_str() {
        Some("status") if args.len() == 2 => transaction_status::show(workspace, output),
        Some("status") => {
            Err("expected latch status <workspace> or latch tx status <workspace>".into())
        }
        Some("begin") if args.len() == 3 => {
            let transaction =
                Transaction::begin(&args[1], &args[2]).map_err(|e| format!("begin: {e}"))?;
            writeln!(
                output,
                "transaction open over {:?}",
                transaction.root().to_string_lossy()
            )
            .map_err(|e| e.to_string())
        }
        Some("changes") if args.len() == 2 => {
            crate::transaction_changes::show_changes(workspace, output)
        }
        Some("commit") => crate::transaction_commit::commit(workspace, args, output),
        Some("rollback") if args.len() == 2 => {
            let mut transaction = open(workspace)?;
            transaction
                .rollback()
                .map_err(|e| format!("rollback: {e}"))?;
            writeln!(output, "rolled back").map_err(|e| e.to_string())
        }
        _ => Err("unsupported command or arguments; run latch --help".into()),
    }
}
