use super::file_audit_test_fixtures::*;
use crate::{AuditSink, FileAudit};
#[test]
fn resume_continues_a_verified_chain_only() {
    let path = temporary("resume");
    let mut sink = FileAudit::create(&path).unwrap();
    sink.append(&event("a1")).unwrap();
    drop(sink);

    let mut sink = FileAudit::resume(&path, 4096).unwrap();
    sink.append(&event("a2")).unwrap();
    let head = sink.head();
    drop(sink);
    let events = FileAudit::read(&path, 4096).unwrap();
    assert_eq!(
        events
            .iter()
            .map(|e| e.action_id.as_str())
            .collect::<Vec<_>>(),
        ["a1", "a2"]
    );
    assert_eq!(FileAudit::verify(&path, 4096).unwrap(), head);

    let mut bytes = std::fs::read(&path).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0x01;
    std::fs::write(&path, &bytes).unwrap();
    assert!(FileAudit::resume(&path, 4096).is_err());
    std::fs::remove_file(path).unwrap();
}
