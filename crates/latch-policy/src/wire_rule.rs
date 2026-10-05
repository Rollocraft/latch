use super::wire_argument_matcher::WireMatcher;
use crate::{Effect, Rule, Selector};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WireRule {
    id: String,
    description: String,
    action: WireSelector,
    resource: WireSelector,
    environment: WireSelector,
    actor: WireSelector,
    #[serde(default)]
    arguments: WireMatcher,
    effect: WireEffect,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireSelector {
    Any {},
    Exact { value: String },
    Prefix { value: String },
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireEffect {
    Allow {},
    Deny {},
    Ask {},
    Limit { budget: String, maximum: u64 },
    Log {},
}

impl From<WireRule> for Rule {
    fn from(rule: WireRule) -> Self {
        Self {
            id: rule.id,
            description: rule.description,
            action: rule.action.into(),
            resource: rule.resource.into(),
            environment: rule.environment.into(),
            actor: rule.actor.into(),
            arguments: rule.arguments.into(),
            effect: match rule.effect {
                WireEffect::Allow {} => Effect::Allow,
                WireEffect::Deny {} => Effect::Deny,
                WireEffect::Ask {} => Effect::Ask,
                WireEffect::Limit { budget, maximum } => Effect::Limit { budget, maximum },
                WireEffect::Log {} => Effect::Log,
            },
        }
    }
}

impl From<WireSelector> for Selector {
    fn from(selector: WireSelector) -> Self {
        match selector {
            WireSelector::Any {} => Self::Any,
            WireSelector::Exact { value } => Self::Exact(value),
            WireSelector::Prefix { value } => Self::Prefix(value),
        }
    }
}
