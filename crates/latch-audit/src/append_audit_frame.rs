use super::{audit_frame_bytes::chain, encode_audit_event::encode};
use crate::{AuditEvent, AuditSink, FileAudit};
use std::io::{self, Write};

impl AuditSink for FileAudit {
    type Error = io::Error;
    fn append(&mut self, event: &AuditEvent) -> io::Result<()> {
        if self.failed {
            return Err(io::Error::other("audit writer failed; recovery required"));
        }
        let payload = encode(event)?;
        let length = u32::try_from(payload.len())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "audit event too large"))?;
        let head = chain(&self.head, length, &payload);
        let result = (|| {
            self.file.write_all(&length.to_le_bytes())?;
            self.file.write_all(&payload)?;
            self.file.write_all(head.bytes())?;
            self.file.sync_all()
        })();
        if result.is_err() {
            self.failed = true;
        } else {
            self.head = head;
        }
        result
    }
}
