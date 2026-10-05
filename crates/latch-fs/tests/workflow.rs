//! Integration tests for the filesystem adapter, grouped by concern. The
//! shared fixtures live in `support`; each module keeps one area of behavior.

#[path = "workflow/adapter_outcomes.rs"]
mod adapter_outcomes;
#[path = "workflow/policy_flow.rs"]
mod policy_flow;
#[path = "workflow/support.rs"]
mod support;
