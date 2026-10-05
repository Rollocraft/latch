//! Copy-on-write filesystem transactions.
//!
//! An agent works against a root directory but its writes land in a private
//! workspace, so the host is unchanged until someone commits. A commit journals
//! the state it replaces before replacing it, which is what makes a rollback
//! after commit exact rather than approximate.
//!
//! This layer mediates writes; it does not confine a process. An agent that can
//! reach the filesystem directly bypasses it, so OS isolation stays the job of
//! the sandbox. What this layer guarantees is that changes made *through* it
//! are visible as a reviewable set and can be undone byte for byte.

mod changes;
mod commit;
mod error;
mod filesystem;
mod inspection;
mod journal;
mod path;
mod restore;
mod snapshot;
mod snapshot_entry;
mod staging;
mod workspace;

pub use changes::{Change, ChangeSet, Selection, Summary};
pub use error::{MAXIMUM_ENTRIES, State, TransactionError};
pub use path::{MAXIMUM_COMPONENTS, MAXIMUM_PATH_BYTES, PathError, RelativePath};
pub use snapshot::{Difference, Entry, EntryKind, Mode, Snapshot};

use error::{BACKUP, MARKER, STAGE, TOMB, VERSION};
use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
};

pub struct Transaction {
    pub(crate) root: PathBuf,
    pub(crate) workspace: PathBuf,
    pub(crate) state: State,
    /// Paths the host did not have when a change replaced them, mirroring the
    /// on-disk journal so that applying a change does not re-read it.
    pub(crate) absent: Vec<(bool, RelativePath)>,
}

impl Transaction {
    /// `root` is the host tree the agent sees. `workspace` must not exist yet
    /// and must lie outside the root, so staged content is never mistaken for
    /// an agent's change to the tree.
    pub fn begin(
        root: impl AsRef<Path>,
        workspace: impl AsRef<Path>,
    ) -> Result<Self, TransactionError> {
        let root = fs::canonicalize(root)?;
        if !root.is_dir() {
            return Err(TransactionError::Io(io::Error::new(
                io::ErrorKind::NotADirectory,
                "transaction root must be a directory",
            )));
        }
        // Decide where the workspace would land before creating it: a rejected
        // transaction must not leave a directory inside the tree it refused.
        let workspace = workspace.as_ref();
        let name = workspace
            .file_name()
            .ok_or(TransactionError::InvalidWorkspace)?;
        let parent = match workspace.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent,
            _ => Path::new("."),
        };
        let workspace = fs::canonicalize(parent)?.join(name);
        if overlaps(&workspace, &root) {
            return Err(TransactionError::WorkspaceInsideRoot);
        }
        match fs::create_dir(&workspace) {
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                return Err(TransactionError::WorkspaceExists);
            }
            other => other?,
        }
        let workspace = fs::canonicalize(&workspace)?;
        // Re-check what the path actually resolved to, in case a component of
        // it was a link into the root.
        if overlaps(&workspace, &root) {
            let _ = fs::remove_dir(&workspace);
            return Err(TransactionError::WorkspaceInsideRoot);
        }
        for area in [STAGE, TOMB, BACKUP] {
            fs::create_dir(workspace.join(area))?;
        }
        let transaction = Self {
            root,
            workspace,
            state: State::Open,
            absent: Vec::new(),
        };
        transaction.write_marker()?;
        Ok(transaction)
    }

    /// Reopen an existing workspace, so that staging and committing can happen
    /// in different processes.
    pub fn open(workspace: impl AsRef<Path>) -> Result<Self, TransactionError> {
        let workspace = fs::canonicalize(workspace)?;
        let marker = fs::read(workspace.join(MARKER))?;
        let text = String::from_utf8(marker).map_err(|_| TransactionError::InvalidWorkspace)?;
        let mut lines = text.splitn(3, '\n');
        if lines.next() != Some(VERSION) {
            return Err(TransactionError::InvalidWorkspace);
        }
        let state = lines
            .next()
            .and_then(State::parse)
            .ok_or(TransactionError::InvalidWorkspace)?;
        let root = lines.next().ok_or(TransactionError::InvalidWorkspace)?;
        Ok(Self {
            root: fs::canonicalize(root)?,
            absent: read_absent(&workspace)?,
            workspace,
            state,
        })
    }

    pub fn state(&self) -> State {
        self.state
    }
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub(crate) fn require_open(&self) -> Result<(), TransactionError> {
        (self.state == State::Open)
            .then_some(())
            .ok_or(TransactionError::NotOpen(self.state))
    }

    pub(crate) fn area(
        &self,
        area: &str,
        path: &RelativePath,
    ) -> Result<PathBuf, TransactionError> {
        Ok(path.resolve(&self.workspace.join(area))?)
    }

    pub(crate) fn write_marker(&self) -> Result<(), TransactionError> {
        workspace::write_marker(&self.workspace, &self.root, self.state.label())
    }
}

use workspace::{overlaps, read_absent};
