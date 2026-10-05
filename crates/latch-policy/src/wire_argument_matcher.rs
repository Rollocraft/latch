use crate::ArgumentMatcher;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum WireMatcher {
    Any {},
    Empty {},
    Exactly { values: Vec<String> },
    Flag { value: String },
    AnyFlag { values: Vec<String> },
    Contains { value: String },
    Positional { index: usize, value: String },
    All { matchers: Vec<WireMatcher> },
}

impl Default for WireMatcher {
    fn default() -> Self {
        Self::Any {}
    }
}

impl From<WireMatcher> for ArgumentMatcher {
    fn from(matcher: WireMatcher) -> Self {
        match matcher {
            WireMatcher::Any {} => Self::Any,
            WireMatcher::Empty {} => Self::Empty,
            WireMatcher::Exactly { values } => Self::Exactly(values),
            WireMatcher::Flag { value } => Self::Flag(value),
            WireMatcher::AnyFlag { values } => Self::AnyFlag(values),
            WireMatcher::Contains { value } => Self::Contains(value),
            WireMatcher::Positional { index, value } => Self::Positional { index, value },
            WireMatcher::All { matchers } => {
                Self::All(matchers.into_iter().map(Self::from).collect())
            }
        }
    }
}
