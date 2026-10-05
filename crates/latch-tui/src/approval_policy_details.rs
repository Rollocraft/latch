use latch_core::Session;
use latch_policy::{Decision, Effect};

use crate::{MAX_ROWS, Output};

/// Matched rules, budget limits, and attached policy listings of one review.
pub(crate) fn policy_details(out: &mut Output, session: &Session, decision: &Decision) {
    out.line("Attached policies:");
    for policy in session.policies().iter().take(MAX_ROWS) {
        out.field("  Policy", policy);
    }
    out.omitted(session.policies().len());
    out.line("Matching policy rules:");
    if decision.matches.is_empty() {
        out.line("  None");
    }
    for rule in decision.matches.iter().take(MAX_ROWS) {
        out.field("  Policy", &rule.policy);
        out.line(&format!("  Level: {:?}", rule.level));
        out.field("  Rule", &rule.rule);
        out.field("  Reason", &rule.description);
        match &rule.effect {
            Effect::Limit { budget, maximum } => {
                out.field("  Effect: limit budget", budget);
                out.line(&format!("  Maximum: {maximum}"));
            }
            effect => out.line(&format!("  Effect: {effect:?}")),
        }
    }
    out.omitted(decision.matches.len());
    out.line("Budget obligations:");
    if decision.limits.is_empty() {
        out.line("  None");
    }
    for (budget, maximum) in decision.limits.iter().take(MAX_ROWS) {
        out.field("  Budget", budget);
        out.line(&format!("  Maximum: {maximum}"));
    }
    out.omitted(decision.limits.len());
}
