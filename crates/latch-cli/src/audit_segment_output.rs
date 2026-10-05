use std::{ffi::OsString, io::Write};

pub(crate) fn audit(path: &OsString, output: &mut impl Write) -> Result<(), String> {
    let events = latch_audit::FileAudit::read(path, 64 * 1024 * 1024).map_err(|e| {
        format!(
            "cannot read audit segment {:?}: {e}",
            path.to_string_lossy()
        )
    })?;
    for event in events {
        // Debug-escaped strings keep embedded newlines and terminal control
        // sequences from masquerading as extra events or terminal commands.
        writeln!(output, "{} {:?} organization={:?} session={:?} owner={:?} agent={:?} action_id={:?} action={:?} resource={:?} environment={:?} risk={} policies={:?}",
            event.timestamp, event.result, event.organization, event.session, event.owner,
            event.agent, event.action_id, event.action, event.resource, event.environment,
            event.risk.value(), event.policies).map_err(|e| e.to_string())?;
    }
    let head = latch_audit::FileAudit::verify(path, 64 * 1024 * 1024).map_err(|e| e.to_string())?;
    writeln!(output, "chain {}", head.to_hex()).map_err(|e| e.to_string())
}
