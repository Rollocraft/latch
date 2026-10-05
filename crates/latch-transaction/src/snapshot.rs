//! A verifiable record of a directory tree, used to prove that a rollback
//! restored exactly the previous state.
//!
//! Covered: directory structure, file contents, permissions and symlink
//! targets. Not covered: timestamps, ownership, extended attributes and
//! hard-link identity. A rollback does not restore those, so claiming them here
//! would only hide the gap.

use crate::path::RelativePath;
use std::{collections::BTreeMap, io, path::Path};

// Kept here so the crate's internal use sites and the public API stay stable.
pub use crate::snapshot_entry::{Entry, EntryKind, Mode};
pub(crate) use crate::snapshot_entry::{digest_of, mode_of, path_error};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    entries: BTreeMap<RelativePath, Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Difference {
    Added(RelativePath),
    Removed(RelativePath),
    Changed {
        path: RelativePath,
        before: Entry,
        after: Entry,
    },
}

impl Snapshot {
    /// Walk a tree without following symbolic links. `maximum_entries` bounds
    /// the work a snapshot of an agent-controlled tree can cause.
    pub fn of(root: impl AsRef<Path>, maximum_entries: usize) -> io::Result<Self> {
        let root = root.as_ref();
        let mut entries = BTreeMap::new();
        let mut pending = vec![root.to_path_buf()];
        while let Some(directory) = pending.pop() {
            for entry in std::fs::read_dir(&directory)? {
                let entry = entry?;
                let path = entry.path();
                let metadata = std::fs::symlink_metadata(&path)?;
                let relative = RelativePath::from_host(root, &path).map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{error:?} at {}", path.display()),
                    )
                })?;
                let kind = classify(&path, &metadata, &mut pending)?;
                entries.insert(
                    relative,
                    Entry {
                        kind,
                        mode: mode_of(&metadata),
                    },
                );
                if entries.len() > maximum_entries {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "tree exceeds snapshot entry limit",
                    ));
                }
            }
        }
        Ok(Self { entries })
    }

    pub fn entries(&self) -> &BTreeMap<RelativePath, Entry> {
        &self.entries
    }

    pub fn get(&self, path: &RelativePath) -> Option<&Entry> {
        self.entries.get(path)
    }

    /// Every way `self` differs from `other`, in path order.
    pub fn differences(&self, other: &Self) -> Vec<Difference> {
        let mut differences = Vec::new();
        for (path, before) in &self.entries {
            match other.entries.get(path) {
                None => differences.push(Difference::Removed(path.clone())),
                Some(after) if after != before => differences.push(Difference::Changed {
                    path: path.clone(),
                    before: before.clone(),
                    after: after.clone(),
                }),
                Some(_) => {}
            }
        }
        for path in other.entries.keys() {
            if !self.entries.contains_key(path) {
                differences.push(Difference::Added(path.clone()));
            }
        }
        differences
    }
}

/// Classify one walked entry, queueing directories for further walking.
fn classify(
    path: &Path,
    metadata: &std::fs::Metadata,
    pending: &mut Vec<std::path::PathBuf>,
) -> io::Result<EntryKind> {
    if metadata.file_type().is_symlink() {
        Ok(EntryKind::Symlink {
            target: std::fs::read_link(path)?.to_string_lossy().into_owned(),
        })
    } else if metadata.is_dir() {
        pending.push(path.to_path_buf());
        Ok(EntryKind::Directory)
    } else if metadata.is_file() {
        Ok(EntryKind::File {
            digest: digest_of(path)?,
            bytes: metadata.len(),
        })
    } else {
        Ok(EntryKind::Other)
    }
}
