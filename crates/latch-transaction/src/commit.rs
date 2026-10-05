use super::{Change, Selection, State, Summary, Transaction, TransactionError};

impl Transaction {
    /// Apply the selected changes to the host, journaling the state each one
    /// replaces. A commit ends the transaction: unselected changes stay in the
    /// workspace for inspection but will not be applied by a later call.
    pub fn commit(&mut self, selection: &Selection) -> Result<Summary, TransactionError> {
        self.require_open()?;
        let changes = self.changes()?;
        let selected = self.select(&changes, selection)?;
        let summary = changes.summary_for(&selected);
        for change in &selected {
            if let Err(error) = self.apply(change) {
                // The journal covers everything applied so far, including this
                // change if it failed midway, so a restore is the safe answer.
                self.restore()?;
                return Err(error);
            }
        }
        self.state = State::Committed;
        self.write_marker()?;
        Ok(summary)
    }

    /// Undo this transaction. Before a commit the host was never touched, so
    /// the staged work is simply discarded. After a commit every journaled path
    /// is restored to the exact content and permissions it was replaced from.
    pub fn rollback(&mut self) -> Result<(), TransactionError> {
        match self.state {
            State::RolledBack => return Ok(()),
            State::Open => {
                for area in [STAGE, TOMB] {
                    let path = self.workspace.join(area);
                    fs::remove_dir_all(&path)?;
                    fs::create_dir(&path)?;
                }
            }
            State::Committed | State::Inconsistent => self.restore()?,
        }
        self.state = State::RolledBack;
        self.write_marker()
    }

    fn select(
        &self,
        changes: &ChangeSet,
        selection: &Selection,
    ) -> Result<Vec<Change>, TransactionError> {
        let Selection::Paths(wanted) = selection else {
            return Ok(changes.changes().to_vec());
        };
        let selected: Vec<Change> = changes
            .changes()
            .iter()
            .filter(|change| change.paths().iter().any(|path| wanted.contains(*path)))
            .cloned()
            .collect();
        // A selection that names nothing real is a mistake worth reporting, not
        // a quiet commit of fewer changes than the caller believed.
        for path in wanted {
            if !selected.iter().any(|change| change.paths().contains(&path)) {
                return Err(TransactionError::UnknownSelection(path.as_str().into()));
            }
        }
        Ok(selected)
    }

    fn apply(&mut self, change: &Change) -> Result<(), TransactionError> {
        match change {
            Change::Created { path, .. } | Change::Modified { path, .. } => self.publish(path),
            Change::Deleted { path, .. } => self.erase(path),
            Change::Renamed { from, to, .. } => {
                self.publish(to)?;
                self.erase(from)
            }
        }
    }
}

use super::{ChangeSet, STAGE, TOMB};
use std::fs;
