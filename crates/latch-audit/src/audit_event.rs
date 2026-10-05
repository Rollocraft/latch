use latch_core::{Action, RiskScore, Session};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    Denied,
    ApprovalRequired,
    Authorized,
    Started,
    Succeeded,
    Failed,
    ReplayRejected,
    BudgetRejected,
    PreparationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub organization: String,
    pub owner: String,
    pub agent: String,
    pub session: String,
    pub action_id: String,
    pub action: String,
    pub resource: String,
    pub environment: String,
    pub risk: RiskScore,
    pub result: EventResult,
    pub policies: Vec<String>,
}

impl AuditEvent {
    /// Attribution comes from the trusted session, even when the request forged
    /// its actor or session fields. Arguments and file contents are not copied.
    pub fn for_action(
        session: &Session,
        action: &Action,
        result: EventResult,
        timestamp: u64,
    ) -> Self {
        Self {
            timestamp,
            organization: session.identity().organization.clone(),
            owner: session.identity().owner.clone(),
            agent: session.identity().id.clone(),
            session: session.id().into(),
            action_id: action.id.clone(),
            action: action.name.clone(),
            resource: action.resource.clone(),
            environment: session.identity().environment.clone(),
            risk: action.risk,
            result,
            policies: session.policies().to_vec(),
        }
    }
}
