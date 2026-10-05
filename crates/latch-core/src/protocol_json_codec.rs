use super::protocol_field_validation::bounded;
use super::protocol_request::RequestDto;
use super::protocol_response::ResponseDto;
use super::{ProtocolError, Request, Response};
use serde::Serialize;

pub fn decode_request(bytes: &[u8]) -> Result<Request, ProtocolError> {
    bounded(bytes)?;
    let dto: RequestDto = serde_json::from_slice(bytes).map_err(|_| ProtocolError::InvalidJson)?;
    dto.try_into()
}

pub fn decode_response(bytes: &[u8]) -> Result<Response, ProtocolError> {
    bounded(bytes)?;
    let dto: ResponseDto = serde_json::from_slice(bytes).map_err(|_| ProtocolError::InvalidJson)?;
    dto.try_into()
}

pub fn encode_request(request: &Request) -> Result<Vec<u8>, ProtocolError> {
    request.validate()?;
    encode(request)
}

pub fn encode_response(response: &Response) -> Result<Vec<u8>, ProtocolError> {
    response.validate()?;
    encode(response)
}

fn encode(value: &impl Serialize) -> Result<Vec<u8>, ProtocolError> {
    let bytes = serde_json::to_vec(value).map_err(|_| ProtocolError::InvalidJson)?;
    bounded(&bytes)?;
    Ok(bytes)
}
