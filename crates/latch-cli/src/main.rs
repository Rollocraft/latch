mod audit_segment_output;
mod bounded_file_read;
mod command_dispatch;
mod command_help;
mod manifest_commands;
mod policy_commands;
mod policy_document_read;
mod policy_explain;
mod policy_hypothesis;
mod scenario_fixture;
mod scenario_help;
mod scenario_identity;
mod scenario_input;
mod scenarios;
mod transaction_changes;
mod transaction_commands;
mod transaction_commit;
mod transaction_status;
mod transaction_workspace;

use command_dispatch::run;
use std::{io, process::ExitCode};

fn main() -> ExitCode {
    match run(
        std::env::args_os().skip(1).collect(),
        &mut io::stdout().lock(),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}: {error}", env!("CARGO_BIN_NAME"));
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod audit_output_tests;
#[cfg(test)]
mod command_argument_tests;
#[cfg(test)]
mod command_test_support;
#[cfg(test)]
mod inspection_test_support;
#[cfg(test)]
mod transaction_workflow_tests;
#[cfg(test)]
mod workflow_tests;

#[cfg(test)]
mod workflow_test_fixtures;

#[cfg(test)]
mod policy_explanation_fixtures;
