use super::wire_rule::WireRule;
use crate::{Policy, PolicyLevel, Rule};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WireDocument {
    pub(super) version: u64,
    pub(super) policies: Vec<WirePolicy>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WirePolicy {
    id: String,
    level: WireLevel,
    rules: Vec<WireRule>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum WireLevel {
    Company,
    Department,
    Team,
    Agent,
    Session,
}

impl From<WirePolicy> for Policy {
    fn from(policy: WirePolicy) -> Self {
        Self {
            id: policy.id,
            level: match policy.level {
                WireLevel::Company => PolicyLevel::Company,
                WireLevel::Department => PolicyLevel::Department,
                WireLevel::Team => PolicyLevel::Team,
                WireLevel::Agent => PolicyLevel::Agent,
                WireLevel::Session => PolicyLevel::Session,
            },
            rules: policy.rules.into_iter().map(Rule::from).collect(),
        }
    }
}
