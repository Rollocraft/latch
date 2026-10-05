//! Metadata-only audit vocabulary. Storage implementations must provide their
//! own durability and integrity guarantees; this model alone provides neither.

mod audit_event;
mod file;
mod memory_audit;
mod query;

pub use audit_event::{AuditEvent, EventResult};
pub use file::FileAudit;
pub use memory_audit::MemoryAudit;
pub use query::*;

/// An implementation must return success only after the event meets its stated
/// durability guarantee. Runtime callers must fail closed on append failure.
pub trait AuditSink {
    type Error;
    fn append(&mut self, event: &AuditEvent) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod memory_scope_tests;
#[cfg(test)]
mod query_tests;
