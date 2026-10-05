use super::file_audit_test_fixtures::*;
use crate::{AuditSink, FileAudit};
#[test]
fn a_poisoned_writer_refuses_further_appends() {
    let path = temporary("poison");
    let mut sink = FileAudit::create(&path).unwrap();
    sink.append(&event("a1")).unwrap();
    let head = sink.head();
    sink.failed = true;
    assert!(sink.append(&event("a2")).is_err());
    assert_eq!(
        sink.head(),
        head,
        "a failed append must not advance the head"
    );
    drop(sink);
    assert_eq!(FileAudit::read(&path, 4096).unwrap().len(), 1);
    std::fs::remove_file(path).unwrap();
}
