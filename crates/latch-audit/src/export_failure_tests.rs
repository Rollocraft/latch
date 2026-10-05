use super::query_test_fixtures::*;
use crate::*;
use std::io::{self, Write};

#[test]
fn export_propagates_partial_write_and_newline_failures() {
    struct LimitedWriter(usize);
    impl Write for LimitedWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.0 == 0 {
                return Err(io::Error::other("synthetic failure"));
            }
            let written = bytes.len().min(self.0);
            self.0 -= written;
            Ok(written)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let events = [event("acme", 0, EventResult::Succeeded)];
    let reader = AuditReader::new(&events);
    let q = query("acme");
    let mut bytes = Vec::new();
    reader
        .export_jsonl(&q, page(0, 1), ResourceExportPolicy::Omit, &mut bytes)
        .unwrap();
    for limit in [0, 1, bytes.len() - 1] {
        assert!(
            reader
                .export_jsonl(
                    &q,
                    page(0, 1),
                    ResourceExportPolicy::Omit,
                    LimitedWriter(limit)
                )
                .is_err()
        );
    }
}
