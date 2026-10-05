use super::protocol_argument_validation::validate_arguments;
use super::protocol_field_validation::{text, version};
use super::{MAX_ID_BYTES, MAX_RESOURCE_BYTES, ProtocolError, Request, RequestedAction};

impl Request {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        version(self.version)?;
        text(&self.request_id, MAX_ID_BYTES, "request_id")?;
        if let Some(id) = &self.idempotency_id {
            text(id, MAX_ID_BYTES, "idempotency_id")?;
        }
        text(&self.caller.tenant, MAX_ID_BYTES, "tenant")?;
        text(&self.caller.actor, MAX_ID_BYTES, "actor")?;
        text(&self.caller.session, MAX_ID_BYTES, "session")?;
        validate_action(&self.action)
    }
}

fn validate_action(action: &RequestedAction) -> Result<(), ProtocolError> {
    text(&action.name, MAX_ID_BYTES, "action.name")?;
    text(&action.resource, MAX_RESOURCE_BYTES, "action.resource")?;
    text(&action.environment, MAX_ID_BYTES, "action.environment")?;
    if action.name.split('.').count() < 2
        || action.name.split('.').any(|part| {
            part.is_empty()
                || !part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        })
    {
        return Err(ProtocolError::InvalidField {
            field: "action.name",
        });
    }
    validate_arguments(&action.arguments)
}
