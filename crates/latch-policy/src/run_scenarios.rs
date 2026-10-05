use super::compare_scenario_decision::compare_scenario_decision;
use crate::{PolicyEngine, PolicyScenario, ScenarioError, ScenarioReport, ScenarioResult};
use std::collections::BTreeSet;

pub fn test_scenarios(
    engine: &PolicyEngine,
    scenarios: &[PolicyScenario<'_>],
) -> Result<ScenarioReport, ScenarioError> {
    let mut ids = BTreeSet::new();
    for scenario in scenarios {
        if !crate::valid_text(&scenario.id) {
            return Err(ScenarioError::InvalidId);
        }
        if !ids.insert(&scenario.id) {
            return Err(ScenarioError::DuplicateId(scenario.id.clone()));
        }
    }
    let results: Vec<_> = scenarios
        .iter()
        .map(|scenario| {
            let input = scenario.input;
            let decision = engine.evaluate(input.session, input.action, input.now);
            let mismatches = compare_scenario_decision(&scenario.expected, &decision);
            ScenarioResult {
                id: scenario.id.clone(),
                decision,
                mismatches,
            }
        })
        .collect();
    let passed = results.iter().filter(|result| result.passed()).count();
    Ok(ScenarioReport {
        passed,
        failed: results.len() - passed,
        results,
    })
}
