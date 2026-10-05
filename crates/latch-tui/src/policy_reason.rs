use latch_policy::Reason;

use crate::Output;

/// Policy decision reason rendered as a stable, quoted line.
pub(crate) fn reason(out: &mut Output, reason: &Reason) {
    match reason {
        Reason::InvalidAction(error) => {
            out.line(&format!("Policy reason: invalid action ({error:?})"))
        }
        Reason::MissingPolicy(policy) => out.field("Policy reason: missing policy", policy),
        Reason::ExplicitDeny => out.line("Policy reason: explicit deny"),
        Reason::DefaultDeny => out.line("Policy reason: default deny; no granting rule"),
        Reason::ApprovalRequired => out.line("Policy reason: human approval required"),
        Reason::BudgetRequired => out.line("Policy reason: budget reservation required"),
        Reason::ExplicitAllow => out.line("Policy reason: explicit allow"),
    }
}
