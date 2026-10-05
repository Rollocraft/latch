use std::{fmt, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    InvalidLimit,
    EmptyFrame,
    FrameTooLarge,
    TruncatedHeader,
    TruncatedPayload,
    InvalidJson,
    Io(io::ErrorKind),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidLimit => "invalid frame limit",
            Self::EmptyFrame => "empty frame",
            Self::FrameTooLarge => "frame exceeds limit",
            Self::TruncatedHeader => "truncated frame header",
            Self::TruncatedPayload => "truncated frame payload",
            Self::InvalidJson => "invalid frame JSON",
            Self::Io(_) => "frame I/O failed",
        })
    }
}

impl std::error::Error for CodecError {}
