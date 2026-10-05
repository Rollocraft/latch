//! Identity and session attribution; validation is not authorization.

#[path = "identity_action.rs"]
mod identity_action;
#[path = "identity_agent.rs"]
mod identity_agent;
#[path = "identity_error.rs"]
mod identity_error;
#[path = "identity_session.rs"]
mod identity_session;
#[path = "identity_session_action.rs"]
mod identity_session_action;
#[path = "identity_text.rs"]
mod identity_text;

pub use identity_action::Action;
pub use identity_agent::AgentIdentity;
pub use identity_error::ValidationError;
pub use identity_session::Session;

#[cfg(test)]
use crate::{Reversibility, RiskScore, SessionState};
#[cfg(test)]
#[path = "identity_test_support.rs"]
mod identity_test_support;
#[cfg(test)]
#[path = "identity_validation_tests.rs"]
mod identity_validation_tests;
#[cfg(test)]
#[path = "session_action_validation_tests.rs"]
mod session_action_validation_tests;
#[cfg(test)]
#[path = "session_attribution_tests.rs"]
mod session_attribution_tests;
