use crate::egress_rule::{specificity, strictness};
use crate::{Decision, Destination, DomainName, Effect, NetworkError, Outcome, Reason, Rule};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Policy {
    pub(crate) rules: Vec<Rule>,
    pub(crate) allow_private_addresses: bool,
}

impl Policy {
    pub fn new(rules: Vec<Rule>) -> Result<Self, NetworkError> {
        let mut ids = BTreeSet::new();
        for rule in &rules {
            if rule.id.trim().is_empty() || !ids.insert(rule.id.clone()) {
                return Err(NetworkError::DuplicateRule);
            }
        }
        Ok(Self {
            rules,
            allow_private_addresses: false,
        })
    }

    /// Permit destinations inside the host's own networks. Only for a runtime
    /// whose whole point is reaching an internal service; it also re-opens the
    /// rebinding path to link-local metadata, so it is never the default.
    pub fn allowing_private_addresses(mut self) -> Self {
        self.allow_private_addresses = true;
        self
    }

    /// Decide about one name in isolation, for dry runs and for explaining a
    /// policy. A connection is decided by [`Policy::evaluate`], which also
    /// accounts for what the name resolved to.
    pub fn evaluate_name(&self, name: &DomainName, destination: &Destination) -> Decision {
        let mut matched = Vec::new();
        let mut best: Option<(u32, Effect, String)> = None;
        for rule in &self.rules {
            let applies = rule.pattern.matches(name)
                && rule
                    .ports
                    .as_ref()
                    .is_none_or(|ports| ports.contains(&destination.port))
                && rule
                    .protocol
                    .is_none_or(|protocol| protocol == destination.protocol);
            if !applies {
                continue;
            }
            matched.push(rule.id.clone());
            let score = specificity(rule);
            let wins = match &best {
                None => true,
                Some((best_score, best_effect, _)) => {
                    score > *best_score
                        || (score == *best_score
                            && strictness(rule.effect) > strictness(*best_effect))
                }
            };
            if wins {
                best = Some((score, rule.effect, rule.id.clone()));
            }
        }
        let (outcome, reason) = match best.as_ref().map(|(_, effect, _)| effect) {
            Some(Effect::Deny) => (Outcome::Deny, Reason::ExplicitDeny),
            Some(Effect::Ask) => (Outcome::Ask, Reason::ApprovalRequired),
            Some(Effect::Allow) => (Outcome::Allow, Reason::ExplicitAllow),
            None => (Outcome::Deny, Reason::DefaultDeny),
        };
        Decision {
            outcome,
            reason,
            names: vec![name.clone()],
            matched,
            decided_by: best.map(|(_, _, id)| id),
        }
    }
}
