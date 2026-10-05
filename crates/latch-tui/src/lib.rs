//! Terminal presentation layer: bounded, escaped, review-only rendering.
//!
//! Every renderer shares byte-bounded output and control-character escaping;
//! none of them executes actions or mutates transaction state.

mod approval_policy_details;
mod approval_response;
mod approval_review;
mod approval_view;
mod bounded_output;
mod policy_reason;
mod render_limits;
mod reversibility_badge;
mod session_context;
mod session_timeline;
mod terminal_text;
mod transaction_selection;

pub use approval_response::{ApprovalResponse, ResponseError};
pub use approval_review::render_approval;
pub use approval_view::ApprovalView;
pub use render_limits::{MAX_FIELD_BYTES, MAX_OUTPUT_BYTES, MAX_RESPONSE_BYTES, MAX_ROWS};
pub use reversibility_badge::reversibility_badge;
pub use session_timeline::render_session_timeline;
pub use terminal_text::{Rendered, terminal_text};
pub use transaction_selection::render_transaction_selection;

pub(crate) use bounded_output::Output;

#[cfg(test)]
mod approval_context_tests;
#[cfg(test)]
mod approval_denial_tests;
#[cfg(test)]
mod approval_response_tests;
#[cfg(test)]
mod output_limits_tests;
#[cfg(test)]
mod presentation_test_fixtures;
#[cfg(test)]
mod reversibility_badge_tests;
#[cfg(test)]
mod terminal_escaping_tests;
#[cfg(test)]
mod timeline_scope_tests;
#[cfg(test)]
mod transaction_selection_tests;
