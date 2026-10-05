use crate::{CodecError, frame_limit::limit};
use serde::de::DeserializeOwned;
use std::io::{self, Read};

pub fn read_json<R: Read + ?Sized, T: DeserializeOwned>(
    reader: &mut R,
    maximum: usize,
) -> Result<Option<T>, CodecError> {
    limit(maximum)?;
    let Some(length) = read_length(reader)? else {
        return Ok(None);
    };
    validate_length(length, maximum)?;
    let mut payload = vec![0; length];
    reader.read_exact(&mut payload).map_err(|error| {
        if error.kind() == io::ErrorKind::UnexpectedEof {
            CodecError::TruncatedPayload
        } else {
            CodecError::Io(error.kind())
        }
    })?;
    serde_json::from_slice(&payload)
        .map(Some)
        .map_err(|_| CodecError::InvalidJson)
}

fn read_length<R: Read + ?Sized>(reader: &mut R) -> Result<Option<usize>, CodecError> {
    let mut header = [0; 4];
    loop {
        match reader.read(&mut header[..1]) {
            Ok(0) => return Ok(None),
            Ok(_) => break,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(CodecError::Io(error.kind())),
        }
    }
    reader.read_exact(&mut header[1..]).map_err(|error| {
        if error.kind() == io::ErrorKind::UnexpectedEof {
            CodecError::TruncatedHeader
        } else {
            CodecError::Io(error.kind())
        }
    })?;
    Ok(Some(u32::from_be_bytes(header) as usize))
}

fn validate_length(length: usize, maximum: usize) -> Result<(), CodecError> {
    if length == 0 {
        return Err(CodecError::EmptyFrame);
    }
    if length > maximum {
        return Err(CodecError::FrameTooLarge);
    }
    Ok(())
}
