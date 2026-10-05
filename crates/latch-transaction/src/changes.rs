use crate::path::{PathError, RelativePath};
use latch_core::Digest;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    Created {
        path: RelativePath,
        digest: Digest,
    },
    /// `before == after` with `permissions` set is a permission-only change.
    Modified {
        path: RelativePath,
        before: Digest,
        after: Digest,
        permissions: bool,
    },
    Deleted {
        path: RelativePath,
        digest: Digest,
    },
    /// A deletion and a creation of byte-identical content, reported as one
    /// move. Only unambiguous pairs are reported: if several files share the
    /// content, each side is reported separately rather than guessed at.
    Renamed {
        from: RelativePath,
        to: RelativePath,
        digest: Digest,
    },
}

impl Change {
    /// The path the change results in, which is the path a selection names.
    pub fn path(&self) -> &RelativePath {
        match self {
            Self::Created { path, .. }
            | Self::Modified { path, .. }
            | Self::Deleted { path, .. } => path,
            Self::Renamed { to, .. } => to,
        }
    }

    /// Every path the change touches. Selecting any of them selects the change.
    pub fn paths(&self) -> Vec<&RelativePath> {
        match self {
            Self::Renamed { from, to, .. } => vec![from, to],
            other => vec![other.path()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Summary {
    pub created: usize,
    pub modified: usize,
    pub deleted: usize,
    pub renamed: usize,
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = Vec::new();
        for (count, label) in [
            (self.modified, "modified"),
            (self.created, "new"),
            (self.deleted, "deleted"),
            (self.renamed, "renamed"),
        ] {
            if count > 0 {
                lines.push(format!(
                    "{count} {label} file{}",
                    if count == 1 { "" } else { "s" }
                ));
            }
        }
        if lines.is_empty() {
            return write!(f, "no changes");
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChangeSet {
    pub(crate) changes: Vec<Change>,
}

impl ChangeSet {
    pub fn changes(&self) -> &[Change] {
        &self.changes
    }
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
    pub fn summary(&self) -> Summary {
        self.summary_for(&self.changes)
    }

    pub(crate) fn summary_for(&self, changes: &[Change]) -> Summary {
        let mut summary = Summary::default();
        for change in changes {
            match change {
                Change::Created { .. } => summary.created += 1,
                Change::Modified { .. } => summary.modified += 1,
                Change::Deleted { .. } => summary.deleted += 1,
                Change::Renamed { .. } => summary.renamed += 1,
            }
        }
        summary
    }
}

/// Which staged changes a commit applies. Naming either side of a rename
/// selects the whole move: half a rename is a different change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    All,
    Paths(BTreeSet<RelativePath>),
}

impl Selection {
    pub fn paths<'a>(paths: impl IntoIterator<Item = &'a str>) -> Result<Self, PathError> {
        Ok(Self::Paths(
            paths
                .into_iter()
                .map(RelativePath::new)
                .collect::<Result<_, _>>()?,
        ))
    }
}
