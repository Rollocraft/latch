use latch_policy::{Decision, Outcome};
use std::{ffi::OsString, io::Write};

pub const HELP: &str = "\
Usage: latch policy explain <policy-file> <action> [resource]
Offline policy evaluation only, not runtime enforcement or audit history.
No event store exists; nothing is executed, approved, or reserved.
Synthetic actor: agent://offline/explain; environment: offline; arguments: [].
Omitted resource defaults to literal unspecified (not a wildcard).
Resources are matched literally, without filesystem access or normalization.
Use latch policy test for explicit identity, environment, and argument fixtures.
";

pub fn explain(args: &[OsString], output: &mut impl Write) -> Result<(), String> {
    if args == [OsString::from("--help")] {
        return output.write_all(HELP.as_bytes()).map_err(|e| e.to_string());
    }
    if !(2..=3).contains(&args.len()) {
        return Err("expected latch policy explain <policy-file> <action> [resource]".into());
    }
    let name = text(&args[1], "action")?;
    let resource = args
        .get(2)
        .map(|r| text(r, "resource"))
        .transpose()?
        .unwrap_or("unspecified");
    let document = crate::policy_document_read::load_policy(&args[0])?;
    let decision = super::policy_hypothesis::evaluate(document, name, resource)?;
    writeln!(
        output,
        "{HELP}\nHypothetical action: {name:?}\nresource: {resource:?}"
    )
    .and_then(|_| print_decision(&decision, output))
    .map_err(|e| format!("cannot write policy explanation: {e}"))
}

fn text<'a>(value: &'a OsString, field: &str) -> Result<&'a str, String> {
    let text = value
        .to_str()
        .ok_or_else(|| format!("{field} must be UTF-8: {value:?}"))?;
    if text.trim().is_empty() || text.chars().any(char::is_control) {
        return Err(format!(
            "{field} must be nonempty and contain no control characters: {value:?}"
        ));
    }
    Ok(text)
}

fn print_decision(decision: &Decision, output: &mut impl Write) -> std::io::Result<()> {
    writeln!(output, "matched rules: {}", decision.matches.len())?;
    for matched in &decision.matches {
        writeln!(
            output,
            "  policy={:?} rule={:?} level={:?} effect={:?} description={:?}",
            matched.policy, matched.rule, matched.level, matched.effect, matched.description
        )?;
    }
    let outcome = match decision.outcome {
        Outcome::Allow => "ALLOW",
        Outcome::Deny => "DENY",
        Outcome::ApprovalRequired => "ASK",
        Outcome::Limited => "LIMIT",
    };
    writeln!(
        output,
        "final outcome: {outcome}\nreason: {:?}\nlimits: {}",
        decision.reason,
        decision.limits.len()
    )?;
    for (budget, maximum) in &decision.limits {
        writeln!(output, "  budget={budget:?} maximum={maximum}")?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "policy_explain_tests.rs"]
mod tests;
