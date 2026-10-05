//! Deterministic local policy evaluation. Decisions describe obligations; only
//! an enforcement backend can satisfy limits or approvals and execute an action.

mod arguments;
pub mod authorization;
mod document;
mod evaluate_action;
mod merge_rule_effects;
mod policy_decision;
mod policy_engine;
mod policy_rules;
mod simulation;
mod validate_policies;

pub use arguments::{ArgumentMatcher, MAXIMUM_DEPTH};
pub use document::*;
pub use policy_decision::{Decision, Outcome, Reason, RuleMatch};
pub use policy_engine::PolicyEngine;
use policy_rules::valid_text;
pub use policy_rules::{Effect, Policy, PolicyError, PolicyLevel, Rule, Selector};
pub use simulation::*;
