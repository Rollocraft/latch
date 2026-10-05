//! SHA-256 (FIPS 180-4) for audit chaining and filesystem integrity.
//!
//! Integrity means tamper evidence, not secrecy or authentication. Whoever can
//! rewrite a chain head can rewrite the chain; callers own authenticated checkpoints.

#[path = "hash_compression.rs"]
mod hash_compression;
#[path = "hash_digest.rs"]
mod hash_digest;
#[path = "hash_sha256.rs"]
mod hash_sha256;

pub use hash_digest::Digest;
pub use hash_sha256::{Sha256, sha256};

#[cfg(test)]
#[path = "hash_encoding_tests.rs"]
mod hash_encoding_tests;
#[cfg(test)]
#[path = "hash_streaming_tests.rs"]
mod hash_streaming_tests;
#[cfg(test)]
#[path = "hash_vectors_tests.rs"]
mod hash_vectors_tests;
