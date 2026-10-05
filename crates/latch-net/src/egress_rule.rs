use crate::{Pattern, Protocol};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub pattern: Pattern,
    /// `None` matches every port. Naming ports is how a rule stays narrow.
    pub ports: Option<BTreeSet<u16>>,
    pub protocol: Option<Protocol>,
    pub effect: Effect,
}

/// How specifically a rule names its destination. The most specific rule
/// decides, which is what makes a closing `deny: "*"` a default rather than a
/// prohibition that would swallow every allow written above it.
///
/// This differs deliberately from `latch-policy`, where a deny always wins:
/// there, rules arrive from a hierarchy of organizational levels, and a company
/// deny must not be overridable by a team. Egress rules have no such hierarchy:
/// one rule set is one author intent, and specificity is how that intent is read.
pub(crate) fn specificity(rule: &Rule) -> u32 {
    let host = match &rule.pattern {
        Pattern::Any => 0,
        Pattern::Subdomains(parent) => parent.labels().count() as u32,
        // An exact name is more specific than a wildcard rooted at that name.
        Pattern::Exact(name) => name.labels().count() as u32 + 1,
    };
    host + u32::from(rule.ports.is_some()) + u32::from(rule.protocol.is_some())
}

/// Ties break toward the stricter effect, so a deny written beside an allow at
/// the same specificity refuses.
pub(crate) fn strictness(effect: Effect) -> u8 {
    match effect {
        Effect::Allow => 0,
        Effect::Ask => 1,
        Effect::Deny => 2,
    }
}
