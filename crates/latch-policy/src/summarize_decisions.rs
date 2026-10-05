use crate::{BudgetSummary, Decision, DryRunSummary, MatchedRule, Outcome, Reason};

pub fn summarize_decisions(decisions: &[Decision]) -> DryRunSummary {
    let mut summary = DryRunSummary {
        total: decisions.len(),
        ..Default::default()
    };
    for decision in decisions {
        match decision.outcome {
            Outcome::Allow => summary.allowed += 1,
            Outcome::Deny => summary.denied += 1,
            Outcome::ApprovalRequired => summary.approval_required += 1,
            Outcome::Limited => summary.limited += 1,
        }
        match decision.reason {
            Reason::ExplicitDeny => summary.explicit_denials += 1,
            Reason::DefaultDeny => summary.default_denials += 1,
            Reason::InvalidAction(_) => summary.invalid_actions += 1,
            Reason::MissingPolicy(_) => summary.missing_policies += 1,
            _ => {}
        }
        if decision
            .matches
            .iter()
            .any(|matched| matched.effect == crate::Effect::Log)
        {
            summary.logged_actions += 1;
        }
        for matched in &decision.matches {
            *summary
                .rule_matches
                .entry(MatchedRule {
                    policy: matched.policy.clone(),
                    rule: matched.rule.clone(),
                })
                .or_default() += 1;
        }
        for (budget, maximum) in &decision.limits {
            let entry = summary
                .budgets
                .entry(budget.clone())
                .or_insert(BudgetSummary {
                    actions: 0,
                    strictest_maximum: *maximum,
                });
            entry.actions += 1;
            entry.strictest_maximum = entry.strictest_maximum.min(*maximum);
        }
    }
    summary
}
