//! The adapter type that owns the transaction its actions stage into.

use crate::actions::{DELETE, RENAME, WRITE};
use crate::errors::FileError;
use latch_runtime::gate::ActionAdapter;
use latch_transaction::Transaction;
use std::collections::BTreeMap;

/// Owns the transaction its actions stage into. The caller keeps the decision
/// to commit or roll back: this type deliberately offers neither.
pub struct FileAdapter {
    transaction: Transaction,
}

impl FileAdapter {
    pub fn new(transaction: Transaction) -> Self {
        Self { transaction }
    }

    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// Take the transaction back to review, commit or roll it back.
    pub fn into_transaction(self) -> Transaction {
        self.transaction
    }
}

/// The payload of an action, separate from the action itself so that an
/// argument is never re-interpreted as part of the path.
pub(crate) fn payload(action: &latch_core::Action) -> Result<&str, FileError> {
    match (action.name.as_str(), action.arguments.as_slice()) {
        (WRITE | RENAME, [argument]) => Ok(argument),
        (DELETE, []) => Ok(""),
        (WRITE | RENAME | DELETE, _) => Err(FileError::MalformedAction),
        (name, _) => Err(FileError::UnknownAction(name.into())),
    }
}

impl ActionAdapter for FileAdapter {
    type Output = ();
    type Error = FileError;

    fn costs(
        &self,
        action: &latch_core::Action,
        limits: &BTreeMap<String, u64>,
    ) -> Result<BTreeMap<String, u64>, Self::Error> {
        crate::pricing::costs(&self.transaction, action, limits)
    }

    fn execute(&mut self, action: &latch_core::Action) -> Result<Self::Output, Self::Error> {
        let payload = payload(action)?;
        match action.name.as_str() {
            WRITE => self
                .transaction
                .write(&action.resource, payload.as_bytes())?,
            DELETE => self.transaction.remove(&action.resource)?,
            RENAME => self.transaction.rename(&action.resource, payload)?,
            name => return Err(FileError::UnknownAction(name.into())),
        }
        Ok(())
    }
}
