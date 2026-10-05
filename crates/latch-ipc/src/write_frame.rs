use crate::{CodecError, frame_limit::limit};
use serde::Serialize;
use std::io::{self, Write};

struct BoundedBuffer {
    bytes: Vec<u8>,
    maximum: usize,
    exceeded: bool,
}

impl Write for BoundedBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.maximum - self.bytes.len() {
            self.exceeded = true;
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn encode<T: Serialize + ?Sized>(value: &T, maximum: usize) -> Result<Vec<u8>, CodecError> {
    let mut buffer = BoundedBuffer {
        bytes: Vec::new(),
        maximum,
        exceeded: false,
    };
    if serde_json::to_writer(&mut buffer, value).is_err() {
        return Err(if buffer.exceeded {
            CodecError::FrameTooLarge
        } else {
            CodecError::InvalidJson
        });
    }
    Ok(buffer.bytes)
}

pub fn write_json<W: Write + ?Sized, T: Serialize + ?Sized>(
    writer: &mut W,
    value: &T,
    maximum: usize,
) -> Result<(), CodecError> {
    limit(maximum)?;
    let bytes = encode(value, maximum)?;
    let header = (bytes.len() as u32).to_be_bytes();
    writer
        .write_all(&header)
        .map_err(|error| CodecError::Io(error.kind()))?;
    writer
        .write_all(&bytes)
        .map_err(|error| CodecError::Io(error.kind()))?;
    Ok(())
}
