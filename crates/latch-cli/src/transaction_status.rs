use latch_transaction::{State, Transaction};
use std::{ffi::OsString, io::Write};

pub fn show(workspace: &OsString, output: &mut impl Write) -> Result<(), String> {
    let transaction = Transaction::open(workspace)
        .map_err(|e| format!("cannot inspect workspace {workspace:?}: {e:?}"))?;
    let summary = transaction
        .changes()
        .map_err(|e| format!("cannot summarize workspace {workspace:?}: {e:?}"))?
        .summary();
    writeln!(
        output,
        "workspace: {:?}\nroot: {:?}\nstate: {}\nstaged change summary:\n{}\n\
         Summary compares the retained stage against the current root; not commit history.",
        transaction.workspace(),
        transaction.root(),
        state_label(transaction.state()),
        summary
    )
    .map_err(|e| format!("cannot write transaction status: {e}"))
}

fn state_label(state: State) -> &'static str {
    match state {
        State::Open => "OPEN",
        State::Committed => "COMMITTED (closed; unselected changes cannot be committed later)",
        State::RolledBack => "ROLLED-BACK (closed)",
        State::Inconsistent => "INCONSISTENT (not open; effects may be partially applied)",
    }
}

#[cfg(test)]
#[path = "transaction_status_tests.rs"]
mod tests;
