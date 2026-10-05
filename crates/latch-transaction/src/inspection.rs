use super::{Change, ChangeSet, Transaction, TransactionError};
use crate::path::RelativePath;
use crate::snapshot::{digest_of, mode_of};
use std::collections::BTreeMap;
use std::fs;

impl Transaction {
    /// Everything staged so far, compared against the host as it is now.
    pub fn changes(&self) -> Result<ChangeSet, TransactionError> {
        let mut created: Vec<(RelativePath, latch_core::Digest)> = Vec::new();
        let mut deleted: Vec<(RelativePath, latch_core::Digest)> = Vec::new();
        let mut changes = Vec::new();
        for relative in walk(&self.workspace.join(STAGE))? {
            let staged = self.area(STAGE, &relative)?;
            let after = digest_of(&staged)?;
            let host = relative.resolve(&self.root)?;
            match fs::symlink_metadata(&host) {
                Ok(metadata) if metadata.is_file() => {
                    let before = digest_of(&host)?;
                    let permissions = mode_of(&metadata) != mode_of(&fs::metadata(&staged)?);
                    if before != after || permissions {
                        changes.push(Change::Modified {
                            path: relative,
                            before,
                            after,
                            permissions,
                        });
                    }
                }
                Ok(_) => return Err(TransactionError::NotAFile(relative)),
                Err(_) => created.push((relative, after)),
            }
        }
        for relative in walk(&self.workspace.join(TOMB))? {
            let host = relative.resolve(&self.root)?;
            if let Ok(metadata) = fs::symlink_metadata(&host) {
                if !metadata.is_file() {
                    return Err(TransactionError::NotAFile(relative));
                }
                deleted.push((relative, digest_of(&host)?));
            }
        }
        let mut counts: BTreeMap<latch_core::Digest, (usize, usize)> = BTreeMap::new();
        for (_, digest) in &created {
            counts.entry(*digest).or_default().0 += 1;
        }
        for (_, digest) in &deleted {
            counts.entry(*digest).or_default().1 += 1;
        }
        let paired = |digest: &Digest| counts.get(digest) == Some(&(1, 1));
        for (to, digest) in &created {
            if paired(digest)
                && let Some((from, _)) = deleted.iter().find(|(_, other)| other == digest)
            {
                changes.push(Change::Renamed {
                    from: from.clone(),
                    to: to.clone(),
                    digest: *digest,
                });
            } else {
                changes.push(Change::Created {
                    path: to.clone(),
                    digest: *digest,
                });
            }
        }
        for (path, digest) in &deleted {
            if !paired(digest) {
                changes.push(Change::Deleted {
                    path: path.clone(),
                    digest: *digest,
                });
            }
        }
        changes.sort_by(|a, b| a.path().cmp(b.path()));
        Ok(ChangeSet { changes })
    }
}

use super::workspace::walk;
use super::{STAGE, TOMB};
use latch_core::Digest;
