/// Whether an action's effects can be undone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reversibility {
    FullyReversible,
    PartiallyReversible,
    Irreversible,
}
