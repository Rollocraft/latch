use std::fmt;

pub const MAX_PAGE_SIZE: usize = 1_000;
pub const MAX_QUERY_TEXT_BYTES: usize = 1_024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryError {
    InvalidText(&'static str),
    InvalidTimeWindow,
    InvalidRiskRange,
    InvalidPage,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidText(field) => write!(f, "invalid query field: {field}"),
            Self::InvalidTimeWindow => f.write_str("time window must have start before end"),
            Self::InvalidRiskRange => f.write_str("minimum risk exceeds maximum risk"),
            Self::InvalidPage => f.write_str("invalid page size or overflowing offset"),
        }
    }
}

impl std::error::Error for QueryError {}
