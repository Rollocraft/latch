//! Bounded, versioned control messages and strict JSON serialization.

#[path = "protocol_action.rs"]
mod protocol_action;
#[path = "protocol_argument_validation.rs"]
mod protocol_argument_validation;
#[path = "protocol_error.rs"]
mod protocol_error;
#[path = "protocol_field_validation.rs"]
mod protocol_field_validation;
#[path = "protocol_json_codec.rs"]
mod protocol_json_codec;
#[path = "protocol_limits.rs"]
mod protocol_limits;
#[path = "protocol_outcome.rs"]
mod protocol_outcome;
#[path = "protocol_request.rs"]
mod protocol_request;
#[path = "protocol_request_validation.rs"]
mod protocol_request_validation;
#[path = "protocol_response.rs"]
mod protocol_response;

pub use protocol_action::{CallerMetadata, RequestedAction};
pub use protocol_error::ProtocolError;
pub use protocol_json_codec::{decode_request, decode_response, encode_request, encode_response};
pub use protocol_limits::*;
pub use protocol_outcome::{Decision, DecisionReason, ErrorCode, Outcome};
pub use protocol_request::Request;
pub use protocol_response::Response;

#[cfg(test)]
#[path = "protocol_argument_bounds_tests.rs"]
mod protocol_argument_bounds_tests;
#[cfg(test)]
#[path = "protocol_json_validation_tests.rs"]
mod protocol_json_validation_tests;
#[cfg(test)]
#[path = "protocol_message_bounds_tests.rs"]
mod protocol_message_bounds_tests;
#[cfg(test)]
#[path = "protocol_request_roundtrip_tests.rs"]
mod protocol_request_roundtrip_tests;
#[cfg(test)]
#[path = "protocol_response_roundtrip_tests.rs"]
mod protocol_response_roundtrip_tests;
#[cfg(test)]
#[path = "protocol_response_validation_tests.rs"]
mod protocol_response_validation_tests;
#[cfg(test)]
#[path = "protocol_test_support.rs"]
mod protocol_test_support;
