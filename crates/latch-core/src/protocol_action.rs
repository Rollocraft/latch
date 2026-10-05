use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallerMetadata {
    pub tenant: String,
    pub actor: String,
    pub session: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedAction {
    pub name: String,
    pub arguments: Vec<String>,
    pub resource: String,
    pub environment: String,
}

impl From<&crate::Action> for RequestedAction {
    fn from(action: &crate::Action) -> Self {
        Self {
            name: action.name.clone(),
            arguments: action.arguments.clone(),
            resource: action.resource.clone(),
            environment: action.environment.clone(),
        }
    }
}
