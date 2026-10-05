use crate::Destination;
use crate::destination::key_of;
use std::collections::BTreeSet;

/// Limits on what may leave a session: uploaded bytes, distinct destinations,
/// and request count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub uploaded_bytes: u64,
    pub destinations: u64,
    pub requests: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetError {
    UploadExhausted,
    TooManyDestinations,
    TooManyRequests,
}

/// Session egress accounting. A destination already paid for is not charged
/// again, so the destination limit counts distinct places, not connections.
#[derive(Debug, Clone)]
pub struct Budget {
    limits: Limits,
    destinations: BTreeSet<String>,
    uploaded: u64,
    requests: u64,
}

impl Budget {
    pub fn new(limits: Limits) -> Self {
        Self {
            limits,
            destinations: BTreeSet::new(),
            uploaded: 0,
            requests: 0,
        }
    }

    pub fn uploaded(&self) -> u64 {
        self.uploaded
    }
    pub fn requests(&self) -> u64 {
        self.requests
    }
    pub fn destinations(&self) -> usize {
        self.destinations.len()
    }

    /// Charge one request before it is made. Nothing is recorded unless every
    /// limit permits it, so a refusal leaves the session exactly as it was.
    pub fn charge(&mut self, destination: &Destination, upload: u64) -> Result<(), BudgetError> {
        let key = key_of(destination);
        let new_destination = !self.destinations.contains(&key);
        let uploaded = self
            .uploaded
            .checked_add(upload)
            .ok_or(BudgetError::UploadExhausted)?;
        if uploaded > self.limits.uploaded_bytes {
            return Err(BudgetError::UploadExhausted);
        }
        let requests = self
            .requests
            .checked_add(1)
            .ok_or(BudgetError::TooManyRequests)?;
        if requests > self.limits.requests {
            return Err(BudgetError::TooManyRequests);
        }
        if new_destination && self.destinations.len() as u64 + 1 > self.limits.destinations {
            return Err(BudgetError::TooManyDestinations);
        }
        self.uploaded = uploaded;
        self.requests = requests;
        self.destinations.insert(key);
        Ok(())
    }
}
