use latch_core::{Digest, Sha256};
use std::io;

pub(super) const HEADER: &[u8; 9] = b"LATCHAUD\x02";

/// The length is bound into the chain so that moving a frame boundary is
/// detected even when the concatenated payload bytes are unchanged.
pub(super) fn chain(previous: &Digest, length: u32, payload: &[u8]) -> Digest {
    let mut hasher = Sha256::new();
    hasher.update(previous.bytes());
    hasher.update(&length.to_le_bytes());
    hasher.update(payload);
    hasher.finish()
}

pub(super) fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
pub(super) fn take<'a>(input: &mut &'a [u8], length: usize) -> io::Result<&'a [u8]> {
    if length > input.len() {
        return Err(invalid("truncated audit record"));
    }
    let (value, rest) = input.split_at(length);
    *input = rest;
    Ok(value)
}
pub(super) fn number(input: &mut &[u8]) -> io::Result<u32> {
    Ok(u32::from_le_bytes(take(input, 4)?.try_into().unwrap()))
}
pub(super) fn text(input: &mut &[u8]) -> io::Result<String> {
    let length = number(input)? as usize;
    String::from_utf8(take(input, length)?.to_vec()).map_err(|_| invalid("invalid UTF-8"))
}

pub(super) fn string(buffer: &mut Vec<u8>, value: &str) -> io::Result<()> {
    let len = u32::try_from(value.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "audit field too large"))?;
    buffer.extend_from_slice(&len.to_le_bytes());
    buffer.extend_from_slice(value.as_bytes());
    Ok(())
}
