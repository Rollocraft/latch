//! Watching for the shape of a data leak.
//!
//! This is a heuristic and is documented as one everywhere it is used. It
//! recognizes two patterns — a secret read followed shortly
//! by an upload to somewhere unfamiliar, and a large volume leaving at once —
//! and it will not recognize an agent that stays below both. Treat what it
//! produces as a reason to ask a human, never as proof that nothing leaked.
//!
//! What it does guarantee is determinism: the same observations in the same
//! order always produce the same response, so a decision can be replayed and
//! argued with afterwards.

#[path = "sensitive_reads.rs"]
mod sensitive_reads;
#[path = "upload_assessment.rs"]
mod upload_assessment;

/// What the runtime should do about a connection. Ordered by severity, which
/// is how several signals about one connection are combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Response {
    Allow,
    Warn,
    RequireApproval,
    Freeze,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensitiveRead {
    pub resource: String,
    pub bytes: u64,
    pub at: u64,
}

/// Correlates sensitive reads with what leaves the session afterwards.
#[derive(Debug, Clone)]
pub struct ExfiltrationMonitor {
    markers: Vec<String>,
    reads: Vec<SensitiveRead>,
    window: u64,
    warn_bytes: u64,
    approval_bytes: u64,
    freeze_bytes: u64,
}

impl Default for ExfiltrationMonitor {
    /// A minute of suspicion after a secret is read, and volume thresholds
    /// chosen to sit above ordinary API traffic and below a bulk copy. Every
    /// deployment should set its own: these are a starting point, not a claim
    /// about what is normal.
    fn default() -> Self {
        Self {
            markers: [
                ".env",
                ".pem",
                ".key",
                "id_rsa",
                "id_ed25519",
                "credentials",
                "secret",
                ".netrc",
                ".npmrc",
                "kubeconfig",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            reads: Vec::new(),
            window: 60,
            warn_bytes: 1 << 20,
            approval_bytes: 10 << 20,
            freeze_bytes: 100 << 20,
        }
    }
}

impl ExfiltrationMonitor {
    /// `markers` are matched case-insensitively against any part of a resource
    /// path, because a secret is usually recognizable by its name.
    pub fn new(markers: Vec<String>, window: u64) -> Self {
        Self {
            markers: markers.iter().map(|m| m.to_ascii_lowercase()).collect(),
            window,
            ..Self::default()
        }
    }

    pub fn with_thresholds(mut self, warn: u64, approval: u64, freeze: u64) -> Self {
        self.warn_bytes = warn;
        self.approval_bytes = approval;
        self.freeze_bytes = freeze;
        self
    }
}

#[cfg(test)]
#[path = "read_retention_tests.rs"]
mod read_retention_tests;
