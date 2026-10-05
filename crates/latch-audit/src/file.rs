//! Hash-chained append-only audit segments.

#[path = "append_audit_frame.rs"]
mod append_audit_frame;
#[path = "audit_frame_bytes.rs"]
mod audit_frame_bytes;
#[path = "decode_audit_event.rs"]
mod decode_audit_event;
#[path = "encode_audit_event.rs"]
mod encode_audit_event;
#[path = "file_audit.rs"]
mod file_audit;
#[path = "scan_audit_segment.rs"]
mod scan_audit_segment;

pub use file_audit::FileAudit;

#[cfg(test)]
#[path = "chain_byte_integrity_tests.rs"]
mod chain_byte_integrity_tests;
#[cfg(test)]
#[path = "chain_order_integrity_tests.rs"]
mod chain_order_integrity_tests;
#[cfg(test)]
#[path = "file_audit_test_fixtures.rs"]
mod file_audit_test_fixtures;
#[cfg(test)]
#[path = "framed_persistence_tests.rs"]
mod framed_persistence_tests;
#[cfg(test)]
#[path = "poisoned_writer_tests.rs"]
mod poisoned_writer_tests;
#[cfg(test)]
#[path = "verified_resume_tests.rs"]
mod verified_resume_tests;
