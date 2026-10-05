//! Integration tests for the execution gate, grouped by concern. The shared
//! fixtures live in `support`; each module keeps one area of behavior.

#[path = "execution/approvals.rs"]
mod approvals;
#[path = "execution/capabilities.rs"]
mod capabilities;
#[path = "execution/capability_audit.rs"]
mod capability_audit;
#[path = "execution/execution_flow.rs"]
mod execution_flow;
#[path = "execution/support.rs"]
mod support;
