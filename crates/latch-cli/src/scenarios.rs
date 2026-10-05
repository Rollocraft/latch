pub use crate::scenario_help::HELP;
use crate::{scenario_fixture::Fixture, scenario_input::Document};
use latch_policy::{DecisionExpectation, EvaluationInput, PolicyEngine, PolicyScenario};
use std::io::Write;

pub fn test(engine: &PolicyEngine, bytes: &[u8], output: &mut impl Write) -> Result<(), String> {
    if bytes.len() > latch_policy::MAX_POLICY_DOCUMENT_BYTES {
        return Err("scenario file exceeds 1 MiB limit".into());
    }
    let document: Document =
        serde_json::from_slice(bytes).map_err(|e| format!("invalid scenario JSON: {e:?}"))?;
    if document.version != 1 {
        return Err("unsupported scenario version; expected 1".into());
    }
    if document.scenarios.is_empty() || document.scenarios.len() > 1024 {
        return Err("expected 1 to 1024 scenarios".into());
    }
    let fixtures = document
        .scenarios
        .into_iter()
        .map(Fixture::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let scenarios = fixtures
        .iter()
        .map(|fixture| PolicyScenario {
            id: fixture.id.clone(),
            input: EvaluationInput {
                session: &fixture.session,
                action: &fixture.action,
                now: fixture.now,
            },
            expected: DecisionExpectation::outcome(fixture.expected),
        })
        .collect::<Vec<_>>();
    let report = latch_policy::test_scenarios(engine, &scenarios)
        .map_err(|e| format!("invalid scenarios: {e:?}"))?;
    for result in &report.results {
        writeln!(
            output,
            "{} {:?}: {:?}; mismatches={:?}",
            if result.passed() { "PASS" } else { "FAIL" },
            result.id,
            result.decision.outcome,
            result.mismatches
        )
        .map_err(|e| e.to_string())?;
    }
    writeln!(output, "{} passed; {} failed", report.passed, report.failed)
        .map_err(|e| e.to_string())?;
    if report.failed != 0 {
        return Err(format!("{} policy scenario(s) failed", report.failed));
    }
    Ok(())
}
