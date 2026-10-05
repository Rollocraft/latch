use super::query_test_fixtures::*;
use crate::*;

#[test]
fn existing_memory_and_verified_file_events_feed_the_same_reader() {
    let path = std::env::temp_dir().join(format!(
        "latch-audit-query-{}-{}.bin",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut file = FileAudit::create(&path).unwrap();
    let mut memory = MemoryAudit::default();
    for e in [
        event("other", 1, EventResult::Denied),
        event("acme", 2, EventResult::Succeeded),
        event("acme", 2, EventResult::Started),
    ] {
        file.append(&e).unwrap();
        memory.append(&e).unwrap();
    }
    drop(file);
    let events = FileAudit::read(&path, 8192).unwrap();
    assert_eq!(events, memory.events());
    let q = query("acme");
    assert_eq!(
        AuditReader::new(&events).timeline(&q, page(0, 10)),
        AuditReader::new(memory.events()).timeline(&q, page(0, 10))
    );
    let before = std::fs::read(&path).unwrap();
    export(&AuditReader::new(&events), &q, ResourceExportPolicy::Redact);
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(&before[..9], b"LATCHAUD\x02");
    std::fs::remove_file(path).unwrap();
}
