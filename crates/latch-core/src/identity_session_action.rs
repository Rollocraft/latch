use super::identity_text::valid_text;
use super::{Action, Session, ValidationError};
use crate::SessionState;

impl Session {
    pub fn validate_action(&self, action: &Action, now: u64) -> Result<(), ValidationError> {
        self.identity().validate_at(now)?;
        if self.lifecycle().state() != SessionState::Active || now < self.lifecycle().updated_at() {
            return Err(ValidationError::InactiveSession);
        }
        self.validate_attribution(action)?;
        validate_action_fields(action)
    }

    fn validate_attribution(&self, action: &Action) -> Result<(), ValidationError> {
        if action.actor != self.identity().id
            || action.session_id != self.id()
            || action.environment != self.identity().environment
        {
            return Err(ValidationError::AttributionMismatch);
        }
        Ok(())
    }
}

fn validate_action_fields(action: &Action) -> Result<(), ValidationError> {
    if !valid_text(&action.id)
        || !valid_text(&action.resource)
        || action.name.split('.').count() < 2
        || action.name.split('.').any(|p| {
            p.is_empty()
                || !p
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        })
    {
        return Err(ValidationError::InvalidAction);
    }
    Ok(())
}
