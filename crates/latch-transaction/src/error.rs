use crate::path::{PathError, RelativePath};
use std::fmt;
use std::io;

pub(crate) const MARKER: &str = "LATCHTX";
pub(crate) const STAGE: &str = "stage";
pub(crate) const TOMB: &str = "tomb";
pub(crate) const BACKUP: &str = "backup";
pub(crate) const ABSENT: &str = "absent.log";
pub(crate) const VERSION: &str = "LATCHTX1";

/// Bounds the work an agent-controlled tree can cause while staging or
/// snapshotting. A tree larger than this must be split across transactions.
pub const MAXIMUM_ENTRIES: usize = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Open,
    Committed,
    RolledBack,
    /// A commit or restore failed partway. Effects may be partially applied and
    /// the journal is still authoritative: `rollback` retries the restore.
    Inconsistent,
}

impl State {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Committed => "committed",
            Self::RolledBack => "rolled-back",
            Self::Inconsistent => "inconsistent",
        }
    }
    pub(crate) fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "open" => Self::Open,
            "committed" => Self::Committed,
            "rolled-back" => Self::RolledBack,
            "inconsistent" => Self::Inconsistent,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum TransactionError {
    Path(PathError),
    Io(io::Error),
    /// The operation needs an open transaction; this one is already finished.
    NotOpen(State),
    WorkspaceExists,
    /// Staging inside the root would make the workspace look like agent changes.
    WorkspaceInsideRoot,
    /// The workspace marker is missing, unreadable or written by another version.
    InvalidWorkspace,
    NotAFile(RelativePath),
    NotFound(RelativePath),
    /// A committed selection named a path with no staged change.
    UnknownSelection(String),
    /// Effects are partially applied and could not be undone. The journal is
    /// retained; this is an incident, not a recoverable error.
    Inconsistent {
        path: RelativePath,
        source: io::Error,
    },
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(error) => write!(f, "invalid path: {error:?}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::NotOpen(state) => write!(f, "transaction is {}", state.label()),
            Self::WorkspaceExists => write!(f, "workspace already exists"),
            Self::WorkspaceInsideRoot => write!(f, "workspace must lie outside the root"),
            Self::InvalidWorkspace => write!(f, "not a latch transaction workspace"),
            Self::NotAFile(path) => write!(f, "not a regular file: {}", path.as_str()),
            Self::NotFound(path) => write!(f, "no such file: {}", path.as_str()),
            Self::UnknownSelection(path) => write!(f, "no staged change for {path:?}"),
            Self::Inconsistent { path, source } => write!(
                f,
                "partially applied transaction: {} could not be restored: {source}",
                path.as_str()
            ),
        }
    }
}

impl std::error::Error for TransactionError {}

impl From<io::Error> for TransactionError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
impl From<PathError> for TransactionError {
    fn from(error: PathError) -> Self {
        Self::Path(error)
    }
}
