use crate::scenario_identity::IdentityInput;
use latch_policy::Outcome;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Document {
    pub(crate) version: u64,
    pub(crate) scenarios: Vec<Scenario>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Scenario {
    pub(crate) id: String,
    pub(crate) now: u64,
    pub(crate) session: SessionInput,
    pub(crate) action: ActionInput,
    pub(crate) expected: Expected,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SessionInput {
    pub(crate) id: String,
    pub(crate) identity: IdentityInput,
    pub(crate) policies: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionInput {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) resource: String,
    pub(crate) arguments: Vec<String>,
    pub(crate) risk: u8,
    pub(crate) reversibility: ReversibilityInput,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReversibilityInput {
    FullyReversible,
    PartiallyReversible,
    Irreversible,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Expected {
    Allow,
    Deny,
    ApprovalRequired,
    Limited,
}

impl From<Expected> for Outcome {
    fn from(input: Expected) -> Self {
        match input {
            Expected::Allow => Self::Allow,
            Expected::Deny => Self::Deny,
            Expected::ApprovalRequired => Self::ApprovalRequired,
            Expected::Limited => Self::Limited,
        }
    }
}
