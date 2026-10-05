//! A policy decision and its budget obligations are checked in one execution
//! path. The caller and adapter are trusted runtime components, not agent code.

use crate::budget::BudgetLedger;
use latch_audit::{AuditEvent, MemoryAudit};
use latch_core::Session;
use latch_policy::PolicyEngine;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[path = "action_adapter.rs"]
mod action_adapter;
#[path = "action_execution.rs"]
mod action_execution;
#[path = "approval_grants.rs"]
mod approval_grants;
#[path = "approval_lifecycle.rs"]
mod approval_lifecycle;
#[path = "approval_quorum.rs"]
mod approval_quorum;
#[path = "approval_records.rs"]
mod approval_records;
#[path = "capability_audit.rs"]
mod capability_audit;
#[path = "capability_execution.rs"]
mod capability_execution;
#[path = "capability_issuance.rs"]
mod capability_issuance;
#[path = "capability_records.rs"]
mod capability_records;
#[path = "capability_token.rs"]
mod capability_token;
#[path = "capability_validation.rs"]
mod capability_validation;
#[path = "execution_authorization.rs"]
mod execution_authorization;
#[path = "execution_budget.rs"]
mod execution_budget;
#[path = "session_lifecycle.rs"]
mod session_lifecycle;

pub use action_adapter::{ActionAdapter, ExecutionError};
pub use approval_quorum::ApprovalQuorum;
pub use approval_records::{ApprovalError, ApprovalRecord};
pub use capability_records::{CapabilityError, CapabilityExecutionError, CapabilityRecord};
pub use capability_token::{CapabilityToken, InvalidCapabilityTokenLength};

/// One gate owns one session and cannot be cloned. Action IDs are consumed before
/// adapter execution, so retries after ambiguous failures cannot repeat effects.
/// Approval issuance must only be exposed to an authenticated approval authority.
pub struct ExecutionGate<S = MemoryAudit> {
    session: Session,
    policies: PolicyEngine,
    budgets: BudgetLedger,
    attempted: BTreeSet<String>,
    approvals: BTreeMap<String, Vec<ApprovalRecord>>,
    approval_history: Vec<ApprovalRecord>,
    approval_quorum: Option<ApprovalQuorum>,
    capabilities: BTreeMap<CapabilityToken, CapabilityRecord>,
    next_capability_id: u64,
    pending_capability_audit: VecDeque<AuditEvent>,
    audit: S,
}

#[cfg(test)]
#[path = "capability_policy_tests.rs"]
mod capability_policy_tests;
#[cfg(test)]
#[path = "capability_scope_tests.rs"]
mod capability_scope_tests;
#[cfg(test)]
#[path = "gate_test_support.rs"]
mod gate_test_support;
