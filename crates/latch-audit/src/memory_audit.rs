use crate::{AuditEvent, AuditSink};

/// Volatile sink for embedding and tests. This is not a persistent audit log.
#[derive(Debug, Default)]
pub struct MemoryAudit {
    events: Vec<AuditEvent>,
}

impl MemoryAudit {
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Scope must be derived from the authenticated reader, not a query supplied
    /// by an agent. Authentication remains the responsibility of the caller.
    pub fn for_session<'a>(
        &'a self,
        organization: &'a str,
        session: &'a str,
    ) -> impl Iterator<Item = &'a AuditEvent> {
        self.events
            .iter()
            .filter(move |event| event.organization == organization && event.session == session)
    }
}

impl AuditSink for MemoryAudit {
    type Error = std::convert::Infallible;
    fn append(&mut self, event: &AuditEvent) -> Result<(), Self::Error> {
        self.events.push(event.clone());
        Ok(())
    }
}
