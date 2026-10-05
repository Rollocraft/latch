use crate::policy_rules::valid_text;
use crate::{Effect, Policy, PolicyError, Rule};
use std::collections::BTreeSet;

pub(crate) fn validate_policies(policies: &[Policy]) -> Result<(), PolicyError> {
    let mut ids = BTreeSet::new();
    for policy in policies {
        if !valid_text(&policy.id) {
            return Err(PolicyError::InvalidPolicy);
        }
        if !ids.insert(&policy.id) {
            return Err(PolicyError::DuplicatePolicy);
        }
        validate_rules(&policy.rules)?;
    }
    Ok(())
}

fn validate_rules(rules: &[Rule]) -> Result<(), PolicyError> {
    let mut ids = BTreeSet::new();
    for rule in rules {
        if !valid_rule(rule) {
            return Err(PolicyError::InvalidPolicy);
        }
        if !ids.insert(&rule.id) {
            return Err(PolicyError::DuplicateRule);
        }
    }
    Ok(())
}

fn valid_rule(rule: &Rule) -> bool {
    valid_text(&rule.id)
        && valid_text(&rule.description)
        && rule.action.valid()
        && rule.resource.valid()
        && rule.environment.valid()
        && rule.actor.valid()
        && rule.arguments.valid()
        && !matches!(&rule.effect, Effect::Limit { budget, .. } if !valid_text(budget))
}
