use super::file_audit_test_fixtures::*;
use crate::{AuditSink, FileAudit};
use latch_core::Digest;
use std::io;
#[test]
fn reordering_and_dropping_events_is_detected() {
    let path = temporary("reorder");
    let mut sink = FileAudit::create(&path).unwrap();
    for id in ["a1", "a2"] {
        sink.append(&event(id)).unwrap();
    }
    drop(sink);
    let bytes = std::fs::read(&path).unwrap();
    let frame = 4 + (bytes.len() - 9 - 2 * Digest::LENGTH - 8) / 2 + Digest::LENGTH;
    let (first, second) = bytes[9..].split_at(frame);
    assert_eq!(second.len(), frame);

    let mut swapped = bytes[..9].to_vec();
    swapped.extend_from_slice(second);
    swapped.extend_from_slice(first);
    let mut dropped = bytes[..9].to_vec();
    dropped.extend_from_slice(second);
    let mut duplicated = bytes.clone();
    duplicated.extend_from_slice(second);
    for corruption in [swapped, dropped, duplicated] {
        std::fs::write(&path, corruption).unwrap();
        assert_eq!(
            FileAudit::read(&path, 8192).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
    std::fs::remove_file(path).unwrap();
}
