use super::{CapabilityError, CapabilityToken, ExecutionGate};
use latch_audit::AuditSink;
use latch_core::Action;

impl<S: AuditSink> ExecutionGate<S> {
    pub(super) fn validate_capability(
        &self,
        token: &CapabilityToken,
        action: &Action,
        now: u64,
    ) -> Result<(), CapabilityError<S::Error>> {
        let record = self
            .capabilities
            .get(token)
            .ok_or(CapabilityError::Unknown)?;
        if record.organization != self.session.identity().organization
            || record.session != self.session.id()
            || record.owner != self.session.identity().owner
            || record.action != *action
        {
            return Err(CapabilityError::ScopeMismatch);
        }
        if record.revoked {
            return Err(CapabilityError::Revoked);
        }
        if record.consumed {
            return Err(CapabilityError::Consumed);
        }
        if now < record.issued_at {
            return Err(CapabilityError::NotYetValid);
        }
        if now >= record.expires_at {
            return Err(CapabilityError::Expired);
        }
        if self.attempted.contains(&action.id) {
            return Err(CapabilityError::AlreadyAttempted);
        }
        Ok(())
    }

    pub(super) fn consume_capabilities(&mut self, action_id: &str) {
        for record in self.capabilities.values_mut() {
            if record.action.id == action_id && !record.revoked {
                record.consumed = true;
            }
        }
    }
}
