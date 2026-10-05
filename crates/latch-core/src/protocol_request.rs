use super::{CallerMetadata, ProtocolError, RequestedAction};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RequestDto")]
pub struct Request {
    pub version: u32,
    pub request_id: String,
    pub idempotency_id: Option<String>,
    pub caller: CallerMetadata,
    pub action: RequestedAction,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RequestDto {
    version: u32,
    request_id: String,
    idempotency_id: Option<String>,
    caller: CallerMetadata,
    action: RequestedAction,
}

impl TryFrom<RequestDto> for Request {
    type Error = ProtocolError;
    fn try_from(dto: RequestDto) -> Result<Self, Self::Error> {
        let request = Self {
            version: dto.version,
            request_id: dto.request_id,
            idempotency_id: dto.idempotency_id,
            caller: dto.caller,
            action: dto.action,
        };
        request.validate()?;
        Ok(request)
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request").finish_non_exhaustive()
    }
}
