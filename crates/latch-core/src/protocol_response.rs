use super::protocol_field_validation::{text, version};
use super::{MAX_ID_BYTES, MAX_REASONS, Outcome, ProtocolError};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ResponseDto")]
pub struct Response {
    pub version: u32,
    pub request_id: Option<String>,
    pub outcome: Outcome,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ResponseDto {
    version: u32,
    request_id: Option<String>,
    outcome: Outcome,
}

impl TryFrom<ResponseDto> for Response {
    type Error = ProtocolError;
    fn try_from(dto: ResponseDto) -> Result<Self, Self::Error> {
        let response = Self {
            version: dto.version,
            request_id: dto.request_id,
            outcome: dto.outcome,
        };
        response.validate()?;
        Ok(response)
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("outcome", &self.outcome)
            .finish_non_exhaustive()
    }
}

impl Response {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        version(self.version)?;
        if let Some(id) = &self.request_id {
            text(id, MAX_ID_BYTES, "request_id")?;
        }
        if matches!(self.outcome, Outcome::Decision { .. }) && self.request_id.is_none() {
            return Err(ProtocolError::InvalidField {
                field: "request_id",
            });
        }
        let reasons = match &self.outcome {
            Outcome::Decision { reasons, .. } | Outcome::Error { reasons, .. } => reasons,
        };
        if reasons.is_empty() {
            return Err(ProtocolError::InvalidField { field: "reasons" });
        }
        if reasons.len() > MAX_REASONS {
            return Err(ProtocolError::LimitExceeded { field: "reasons" });
        }
        Ok(())
    }
}
