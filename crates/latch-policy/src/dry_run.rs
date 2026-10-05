use crate::{DryRunReport, EvaluationInput, PolicyEngine, summarize_decisions};

pub fn dry_run(engine: &PolicyEngine, inputs: &[EvaluationInput<'_>]) -> DryRunReport {
    let decisions: Vec<_> = inputs
        .iter()
        .map(|input| engine.evaluate(input.session, input.action, input.now))
        .collect();
    let summary = summarize_decisions(&decisions);
    DryRunReport { summary, decisions }
}
