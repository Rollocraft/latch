//! Filesystem actions, executed through a transaction.
//!
//! This is the adapter the execution gate drives once a policy has allowed an
//! action: it turns a semantic action such as `file.write` into staged work
//! that a human can review, commit or undo. It never decides anything itself.
//! Deciding is the policy engine's job, charging the budget is the gate's, and
//! this adapter only refuses what it cannot represent.

mod actions;
mod adapter;
mod errors;
mod pricing;

pub use actions::{BYTES, DELETE, FILES, RENAME, WRITE};
pub use adapter::FileAdapter;
pub use errors::FileError;
