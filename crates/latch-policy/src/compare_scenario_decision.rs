use crate::{Decision, DecisionExpectation, MatchedRule, ScenarioMismatch};

pub(crate) fn compare_scenario_decision(
    expected: &DecisionExpectation,
    decision: &Decision,
) -> Vec<ScenarioMismatch> {
    let mut mismatches = Vec::new();
    if expected.outcome != decision.outcome {
        mismatches.push(ScenarioMismatch::Outcome {
            expected: expected.outcome,
            actual: decision.outcome,
        });
    }
    if let Some(reason) = &expected.reason
        && reason != &decision.reason
    {
        mismatches.push(ScenarioMismatch::Reason {
            expected: reason.clone(),
            actual: decision.reason.clone(),
        });
    }
    if let Some(rules) = &expected.matched_rules {
        let actual = decision
            .matches
            .iter()
            .map(|matched| MatchedRule {
                policy: matched.policy.clone(),
                rule: matched.rule.clone(),
            })
            .collect::<Vec<_>>();
        if rules != &actual {
            mismatches.push(ScenarioMismatch::MatchedRules {
                expected: rules.clone(),
                actual,
            });
        }
    }
    if let Some(limits) = &expected.limits
        && limits != &decision.limits
    {
        mismatches.push(ScenarioMismatch::Limits {
            expected: limits.clone(),
            actual: decision.limits.clone(),
        });
    }
    mismatches
}
