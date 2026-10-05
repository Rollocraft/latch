//! Budget pricing for file actions: the cost estimate the gate charges before
//! the adapter is allowed to run.

use crate::actions::{BYTES, FILES, RENAME, WRITE};
use crate::adapter::payload;
use crate::errors::FileError;
use latch_core::Action;
use latch_transaction::Transaction;
use std::collections::BTreeMap;

pub(crate) fn costs(
    transaction: &Transaction,
    action: &Action,
    limits: &BTreeMap<String, u64>,
) -> Result<BTreeMap<String, u64>, FileError> {
    let payload = payload(action)?;
    let mut costs = BTreeMap::new();
    for name in limits.keys() {
        let cost = match (name.as_str(), action.name.as_str()) {
            (BYTES, WRITE) => payload.len() as u64,
            // A move restages the content, so it is charged for it. Reading
            // the source is the only way to bound that without guessing.
            (BYTES, RENAME) => transaction
                .read(&action.resource)
                .map(|contents| contents.len() as u64)?,
            (BYTES, _) => 0,
            (FILES, _) => 1,
            (other, _) => return Err(FileError::UnknownBudget(other.into())),
        };
        costs.insert(name.clone(), cost);
    }
    Ok(costs)
}
