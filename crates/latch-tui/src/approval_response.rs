use crate::MAX_RESPONSE_BYTES;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ApprovalResponse {
    AllowOnce,
    #[default]
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseError {
    TooLong,
    InvalidInput,
    AllowUnavailable,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::TooLong => "approval response is too long",
            Self::InvalidInput => "expected allow once or deny; empty input means deny",
            Self::AllowUnavailable => "allow once is unavailable for this review",
        })
    }
}

impl std::error::Error for ResponseError {}

impl FromStr for ApprovalResponse {
    type Err = ResponseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() > MAX_RESPONSE_BYTES {
            return Err(ResponseError::TooLong);
        }
        let input = input
            .strip_suffix("\r\n")
            .or_else(|| input.strip_suffix('\n'))
            .unwrap_or(input);
        if !input.bytes().all(|b| b.is_ascii_graphic() || b == b' ') {
            return Err(ResponseError::InvalidInput);
        }
        let input = input.trim_matches(' ');
        if input.is_empty() || input.eq_ignore_ascii_case("deny") {
            Ok(Self::Deny)
        } else if input.eq_ignore_ascii_case("allow once") {
            Ok(Self::AllowOnce)
        } else {
            Err(ResponseError::InvalidInput)
        }
    }
}
