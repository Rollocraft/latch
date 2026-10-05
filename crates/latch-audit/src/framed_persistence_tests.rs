use super::audit_frame_bytes::HEADER;
use super::file_audit_test_fixtures::*;
use crate::{AuditSink, FileAudit};
use latch_core::{Digest, sha256};
use std::io;
#[test]
fn persists_framed_events_and_refuses_overwrite() {
    let path = temporary("frames");
    let event = event("a1");
    let mut sink = FileAudit::create(&path).unwrap();
    sink.append(&event).unwrap();
    assert!(FileAudit::create(&path).is_err());
    let head = sink.head();
    drop(sink);
    assert_eq!(FileAudit::read(&path, 4096).unwrap(), vec![event]);
    assert_eq!(FileAudit::verify(&path, 4096).unwrap(), head);
    assert!(FileAudit::read(&path, 8).is_err());
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[..9], HEADER);
    let length = u32::from_le_bytes(bytes[9..13].try_into().unwrap()) as usize;
    assert_eq!(length, bytes.len() - 13 - Digest::LENGTH);
    assert_eq!(u64::from_le_bytes(bytes[13..21].try_into().unwrap()), 42);
    assert_eq!(&bytes[bytes.len() - Digest::LENGTH..], head.bytes());
    for length in (0..bytes.len()).filter(|length| *length != 9) {
        std::fs::write(&path, &bytes[..length]).unwrap();
        assert!(FileAudit::read(&path, 4096).is_err());
    }
    // A complete header with no frames is a valid empty segment.
    std::fs::write(&path, &bytes[..9]).unwrap();
    assert!(FileAudit::read(&path, 4096).unwrap().is_empty());
    assert_eq!(FileAudit::verify(&path, 4096).unwrap(), sha256(HEADER));

    let mut cursor = 21;
    for _ in 0..8 {
        let field_length =
            u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4 + field_length;
    }
    let count_offset = cursor + 2;
    let mut corruptions = Vec::new();
    let mut invalid_count = bytes.clone();
    invalid_count[count_offset..count_offset + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    corruptions.push(invalid_count);
    let mut trailing = bytes.clone();
    trailing.push(0);
    trailing[9..13].copy_from_slice(&((length + 1) as u32).to_le_bytes());
    corruptions.push(trailing);
    for corruption in corruptions {
        std::fs::write(&path, corruption).unwrap();
        assert_eq!(
            FileAudit::read(&path, 4096).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
    std::fs::remove_file(path).unwrap();
}
