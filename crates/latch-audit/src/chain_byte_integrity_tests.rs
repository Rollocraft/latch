use super::file_audit_test_fixtures::*;
use crate::{AuditSink, FileAudit};
#[test]
fn every_byte_of_the_chain_is_covered() {
    let path = temporary("tamper");
    let mut sink = FileAudit::create(&path).unwrap();
    for id in ["a1", "a2", "a3"] {
        sink.append(&event(id)).unwrap();
    }
    drop(sink);
    let bytes = std::fs::read(&path).unwrap();
    for offset in 0..bytes.len() {
        let mut corrupted = bytes.clone();
        corrupted[offset] ^= 0x01;
        assert_eq!(
            FileAudit::read(&path, 4096).unwrap().len(),
            3,
            "original segment must stay readable"
        );
        std::fs::write(&path, &corrupted).unwrap();
        assert!(
            FileAudit::read(&path, 4096).is_err(),
            "flipping byte {offset} stayed undetected"
        );
        std::fs::write(&path, &bytes).unwrap();
    }
    std::fs::remove_file(path).unwrap();
}
