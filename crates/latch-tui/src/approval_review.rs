use latch_core::{Action, Session, SessionState};
use latch_policy::{Decision, Effect, Outcome, Reason};

use crate::{ApprovalView, Output, reversibility_badge};

use crate::{
    approval_policy_details::policy_details, policy_reason::reason,
    session_context::session_context,
};

/// Renders the human approval review; presentation only, never execution.
pub fn render_approval(session: &Session, action: &Action, decision: &Decision) -> ApprovalView {
    let mut out = Output::default();
    out.line("APPROVAL REVIEW - presentation only; no action is executed");
    session_context(&mut out, session);
    out.field("Action ID", &action.id);
    out.field("Action", &action.name);
    out.field("Resource", &action.resource);
    out.line(reversibility_badge(action.reversibility));
    out.line(&format!("Risk: {}/100", action.risk.value()));
    out.line(&format!("Policy outcome: {:?}", decision.outcome));
    reason(&mut out, &decision.reason);
    let attributed = action.session_id == session.id()
        && action.actor == session.identity().id
        && action.environment == session.identity().environment;
    if !attributed {
        out.line("[INVALID CONTEXT] Action attribution does not match the trusted session");
    }
    policy_details(&mut out, session, decision);
    out.line("Arguments and payload contents: hidden");
    let allow_once_available = attributed
        && session.lifecycle().state() == SessionState::Active
        && decision.outcome == Outcome::ApprovalRequired
        && decision.reason == Reason::ApprovalRequired
        && !out.truncated
        && !decision
            .matches
            .iter()
            .any(|rule| rule.effect == Effect::Deny);
    if allow_once_available {
        out.line("Response: allow once / deny [default: deny]");
    } else {
        out.line("Response: deny [default: deny]; allow once unavailable");
    }
    out.line("Response is intent only; the runtime must revalidate and authorize execution.");
    let rendered = out.finish();
    ApprovalView {
        allow_once_available: allow_once_available && !rendered.truncated,
        rendered,
    }
}
