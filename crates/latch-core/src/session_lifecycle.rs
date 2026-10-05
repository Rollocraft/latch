#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Frozen,
    Committed,
    RolledBack,
    Failed,
}

/// Lifecycle bookkeeping only: enforcement backends must perform the actual
/// freeze, commit or rollback before recording the corresponding transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionLifecycle {
    state: SessionState,
    started_at: u64,
    updated_at: u64,
    ended_at: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    TimeReversed,
    InvalidTransition,
}

impl SessionLifecycle {
    /// Timestamps are Unix seconds supplied by the trusted runtime.
    pub fn new(now: u64) -> Self {
        Self {
            state: SessionState::Active,
            started_at: now,
            updated_at: now,
            ended_at: None,
        }
    }
    pub fn state(&self) -> SessionState {
        self.state
    }
    pub fn started_at(&self) -> u64 {
        self.started_at
    }
    pub fn ended_at(&self) -> Option<u64> {
        self.ended_at
    }
    pub(crate) fn updated_at(&self) -> u64 {
        self.updated_at
    }

    pub fn transition(&mut self, next: SessionState, now: u64) -> Result<(), TransitionError> {
        use SessionState::*;
        if now < self.updated_at {
            return Err(TransitionError::TimeReversed);
        }
        if !matches!(
            (self.state, next),
            (Active, Frozen | Committed | RolledBack | Failed)
                | (Frozen, Active | RolledBack | Failed)
        ) {
            return Err(TransitionError::InvalidTransition);
        }
        self.state = next;
        self.updated_at = now;
        if matches!(next, Committed | RolledBack | Failed) {
            self.ended_at = Some(now);
        }
        Ok(())
    }
}
