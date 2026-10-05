use latch_audit::AuditPage;
use latch_core::Session;

use crate::{MAX_ROWS, Output, Rendered, session_context::session_context};

/// Renders one audit page for one session; foreign events are hidden, never shown.
pub fn render_session_timeline(session: &Session, page: &AuditPage<'_>) -> Rendered {
    let mut out = Output::default();
    out.line("AUDIT SESSION TIMELINE - supplied page order; no integrity verification");
    session_context(&mut out, session);
    if page.events.is_empty() {
        out.line("No events on this page");
    }
    for event in page.events.iter().take(MAX_ROWS) {
        if event.organization != session.identity().organization || event.session != session.id() {
            out.line("[SCOPE MISMATCH] Event omitted");
            continue;
        }
        out.line(&format!(
            "Time: {} Unix seconds | Result: {:?} | Risk: {}/100",
            event.timestamp,
            event.result,
            event.risk.value()
        ));
        out.field("  Action ID", &event.action_id);
        out.field("  Action", &event.action);
        out.field("  Resource", &event.resource);
        out.field("  Human owner", &event.owner);
        out.field("  Agent", &event.agent);
        out.field("  Environment", &event.environment);
        for policy in event.policies.iter().take(MAX_ROWS) {
            out.field("  Policy", policy);
        }
        out.omitted(event.policies.len());
    }
    out.omitted(page.events.len());
    if let Some(offset) = page.next_offset {
        out.line(&format!("More query results; next offset: {offset}"));
    }
    out.finish()
}
