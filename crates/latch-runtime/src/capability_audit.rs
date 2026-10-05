use super::{CapabilityError, CapabilityRecord, ExecutionGate};
use latch_audit::{AuditEvent, AuditSink, EventResult};

impl<S: AuditSink> ExecutionGate<S> {
    pub fn capabilities(&self) -> impl Iterator<Item = &CapabilityRecord> {
        self.capabilities.values()
    }

    pub(super) fn capability_event(
        &self,
        record: &CapabilityRecord,
        operation: &str,
        result: EventResult,
        now: u64,
    ) -> AuditEvent {
        let mut event = AuditEvent::for_action(&self.session, &record.action, result, now);
        event.action = operation.into();
        event.resource = format!("capability/{}", record.id);
        event
    }

    pub(super) fn queue_capability_revocation(&mut self, record: &CapabilityRecord, now: u64) {
        let event = self.capability_event(
            record,
            "runtime.capability.revoke",
            EventResult::Denied,
            now,
        );
        self.pending_capability_audit.push_back(event);
    }

    pub fn flush_capability_audit(&mut self) -> Result<(), S::Error> {
        while let Some(event) = self.pending_capability_audit.front() {
            self.audit.append(event)?;
            self.pending_capability_audit.pop_front();
        }
        Ok(())
    }

    pub fn pending_capability_audit(&self) -> impl Iterator<Item = &AuditEvent> {
        self.pending_capability_audit.iter()
    }

    pub fn revoke_capability(
        &mut self,
        id: u64,
        now: u64,
    ) -> Result<bool, CapabilityError<S::Error>> {
        let record = self
            .capabilities
            .values_mut()
            .find(|record| record.id == id);
        let revoked =
            if let Some(record) = record.filter(|record| !record.revoked && !record.consumed) {
                record.revoked = true;
                let record = record.clone();
                self.queue_capability_revocation(&record, now);
                true
            } else {
                false
            };
        self.flush_capability_audit()
            .map_err(CapabilityError::Audit)?;
        Ok(revoked)
    }
}
