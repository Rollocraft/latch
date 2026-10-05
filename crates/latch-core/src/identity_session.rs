use super::identity_text::valid_text;
use super::{AgentIdentity, ValidationError};
use crate::{SessionLifecycle, SessionState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id: String,
    identity: AgentIdentity,
    policies: Vec<String>,
    lifecycle: SessionLifecycle,
}

impl Session {
    pub fn new(
        id: String,
        identity: AgentIdentity,
        policies: Vec<String>,
        now: u64,
    ) -> Result<Self, ValidationError> {
        identity.validate_at(now)?;
        if !valid_text(&id) || policies.is_empty() || policies.iter().any(|p| !valid_text(p)) {
            return Err(ValidationError::MissingField);
        }
        Ok(Self {
            id,
            identity,
            policies,
            lifecycle: SessionLifecycle::new(now),
        })
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn identity(&self) -> &AgentIdentity {
        &self.identity
    }
    pub fn policies(&self) -> &[String] {
        &self.policies
    }
    pub fn lifecycle(&self) -> &SessionLifecycle {
        &self.lifecycle
    }

    pub fn transition(&mut self, next: SessionState, now: u64) -> Result<(), ValidationError> {
        if next == SessionState::Active {
            self.identity.validate_at(now)?;
        }
        self.lifecycle
            .transition(next, now)
            .map_err(ValidationError::Transition)
    }
}
