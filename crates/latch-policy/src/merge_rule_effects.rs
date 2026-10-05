use crate::{Decision, Effect, Outcome, Policy, Reason, RuleMatch};
use latch_core::Action;

pub(crate) fn merge_rule_effects(policies: &[Policy], action: &Action, decision: &mut Decision) {
    let (mut allow, mut deny, mut ask) = (false, false, false);
    for policy in policies {
        for rule in &policy.rules {
            if !rule.matches(action) {
                continue;
            }
            decision.matches.push(RuleMatch {
                policy: policy.id.clone(),
                level: policy.level,
                rule: rule.id.clone(),
                description: rule.description.clone(),
                effect: rule.effect.clone(),
            });
            match &rule.effect {
                Effect::Deny => deny = true,
                Effect::Ask => ask = true,
                Effect::Allow => allow = true,
                Effect::Limit { budget, maximum } => {
                    decision
                        .limits
                        .entry(budget.clone())
                        .and_modify(|n| *n = (*n).min(*maximum))
                        .or_insert(*maximum);
                }
                Effect::Log => {}
            }
        }
    }
    let (outcome, reason) = if deny {
        (Outcome::Deny, Reason::ExplicitDeny)
    } else if ask {
        (Outcome::ApprovalRequired, Reason::ApprovalRequired)
    } else if !allow {
        (Outcome::Deny, Reason::DefaultDeny)
    } else if !decision.limits.is_empty() {
        (Outcome::Limited, Reason::BudgetRequired)
    } else {
        (Outcome::Allow, Reason::ExplicitAllow)
    };
    decision.outcome = outcome;
    decision.reason = reason;
}
