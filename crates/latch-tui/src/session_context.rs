use latch_core::Session;

use crate::Output;

/// Trusted session attribution lines shared by every review renderer.
pub(crate) fn session_context(out: &mut Output, session: &Session) {
    out.field("Session", session.id());
    out.field("Organization", &session.identity().organization);
    out.field("Human owner", &session.identity().owner);
    out.field("Agent", &session.identity().id);
    out.field("Environment", &session.identity().environment);
    out.line(&format!("Session state: {:?}", session.lifecycle().state()));
}
