use latch_core::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalRecord {
    pub action: Action,
    pub approver: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub consumed: bool,
    pub revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalError {
    InvalidApprover,
    InvalidExpiration,
    NotPending,
    AlreadyApproved,
    ActionChanged,
    InvalidQuorum,
    UnauthorizedApprover,
    ApproveAuditFailed,
}
