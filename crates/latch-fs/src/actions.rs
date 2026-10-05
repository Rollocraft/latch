//! Semantic action names and budget names.

/// Semantic action names, provider-independent by design: the same names must
/// describe the same effect whichever agent runtime produced them.
pub const WRITE: &str = "file.write";
pub const DELETE: &str = "file.delete";
pub const RENAME: &str = "file.rename";

/// Budget names this adapter can price. A policy naming any other budget for a
/// file action is a configuration error, not a licence to skip accounting.
pub const BYTES: &str = "bytes";
pub const FILES: &str = "files";
