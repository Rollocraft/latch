//! Integration tests for transaction integrity, grouped by concern. The
//! shared fixture lives in `support`; each module keeps one area of behavior.

#[path = "integrity/support.rs"]
mod support;

#[path = "integrity/staging_visibility.rs"]
mod staging_visibility;

#[path = "integrity/commit_restore.rs"]
mod commit_restore;

#[path = "integrity/selection.rs"]
mod selection;

#[path = "integrity/workspace_isolation.rs"]
mod workspace_isolation;
