use super::{audit_frame_bytes::*, decode_audit_event::decode};
use crate::AuditEvent;
use latch_core::{Digest, sha256};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub(super) fn scan(
    path: impl AsRef<Path>,
    maximum_bytes: usize,
) -> io::Result<(Vec<AuditEvent>, Digest)> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take((maximum_bytes as u64).saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() > maximum_bytes {
        return Err(invalid("audit segment exceeds read limit"));
    }
    if !bytes.starts_with(HEADER) {
        return Err(invalid("unsupported audit header"));
    }
    let mut input = &bytes[HEADER.len()..];
    let mut head = sha256(HEADER);
    let mut events = Vec::new();
    while !input.is_empty() {
        let length = number(&mut input)? as usize;
        let payload = take(&mut input, length)?;
        let recorded = take(&mut input, Digest::LENGTH)?;
        head = chain(&head, length as u32, payload);
        if recorded != head.bytes() {
            return Err(invalid("audit chain broken"));
        }
        events.push(decode(payload)?);
    }
    Ok((events, head))
}
