use latch_core::Reversibility;

/// Honest reversibility badge; wording is part of the reviewed presentation.
pub fn reversibility_badge(value: Reversibility) -> &'static str {
    match value {
        Reversibility::FullyReversible => "[REVERSIBLE] Effects can be undone",
        Reversibility::PartiallyReversible => {
            "[PARTIALLY REVERSIBLE] Some effects cannot be undone"
        }
        Reversibility::Irreversible => "[IRREVERSIBLE] Effects cannot be undone",
    }
}
