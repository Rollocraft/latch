use super::{CapabilityError, CapabilityRecord, CapabilityToken, ExecutionGate};
use latch_audit::{AuditSink, EventResult};
use latch_core::Action;

impl<S: AuditSink> ExecutionGate<S> {
    pub fn issue_capability(
        &mut self,
        action: &Action,
        issuer: String,
        now: u64,
        expires_at: u64,
    ) -> Result<CapabilityToken, CapabilityError<S::Error>> {
        self.validate_issuance(&issuer, now, expires_at)?;
        self.flush_capability_audit()
            .map_err(CapabilityError::Audit)?;
        if self.attempted.contains(&action.id) {
            return Err(CapabilityError::AlreadyAttempted);
        }
        self.authorize_capability(action, now)?;
        let next = self
            .next_capability_id
            .checked_add(1)
            .ok_or(CapabilityError::IdentifierExhausted)?;
        let token = CapabilityToken::generate().map_err(CapabilityError::Random)?;
        if self.capabilities.contains_key(&token) {
            return Err(CapabilityError::TokenCollision);
        }
        let record = self.capability_record(action, issuer, now, expires_at);
        self.next_capability_id = next;
        self.record_capability_issue(&token, record, now)?;
        Ok(token)
    }

    fn validate_issuance(
        &self,
        issuer: &str,
        now: u64,
        expires_at: u64,
    ) -> Result<(), CapabilityError<S::Error>> {
        if issuer.trim().is_empty() || issuer.chars().any(char::is_control) {
            return Err(CapabilityError::InvalidIssuer);
        }
        let identity = self.session.identity();
        if now < identity.created_at || expires_at <= now || expires_at > identity.expires_at {
            return Err(CapabilityError::InvalidExpiration);
        }
        Ok(())
    }

    fn capability_record(
        &self,
        action: &Action,
        issuer: String,
        now: u64,
        expires_at: u64,
    ) -> CapabilityRecord {
        CapabilityRecord {
            id: self.next_capability_id,
            action: action.clone(),
            organization: self.session.identity().organization.clone(),
            session: self.session.id().into(),
            owner: self.session.identity().owner.clone(),
            issuer,
            issued_at: now,
            expires_at,
            revoked: false,
            consumed: false,
        }
    }

    fn record_capability_issue(
        &mut self,
        token: &CapabilityToken,
        record: CapabilityRecord,
        now: u64,
    ) -> Result<(), CapabilityError<S::Error>> {
        let event = self.capability_event(
            &record,
            "runtime.capability.issue",
            EventResult::Authorized,
            now,
        );
        let stored_token = CapabilityToken::import_secret(&token.export_secret())
            .expect("generated token has fixed length");
        self.capabilities.insert(stored_token, record);
        if let Err(error) = self.audit.append(&event) {
            let record = self
                .capabilities
                .get_mut(token)
                .expect("inserted capability");
            record.revoked = true;
            let record = record.clone();
            self.queue_capability_revocation(&record, now);
            return Err(CapabilityError::Audit(error));
        }
        Ok(())
    }
}
