use super::{State, Transaction, TransactionError};
use crate::path::RelativePath;
use crate::snapshot::path_error;
use std::{fs, io};

impl Transaction {
    /// Put every journaled path back exactly as it was found.
    pub(crate) fn restore(&mut self) -> Result<(), TransactionError> {
        let fail = |transaction: &mut Self, path: &RelativePath, source: io::Error| {
            transaction.state = State::Inconsistent;
            let _ = transaction.write_marker();
            TransactionError::Inconsistent {
                path: path.clone(),
                source,
            }
        };
        self.restore_backups(&fail)?;
        self.remove_recorded_absents(&fail)
    }

    fn restore_backups(
        &mut self,
        fail: &dyn Fn(&mut Self, &RelativePath, io::Error) -> TransactionError,
    ) -> Result<(), TransactionError> {
        for relative in walk(&self.workspace.join(super::BACKUP))? {
            let backup = self.area(super::BACKUP, &relative)?;
            let host = relative.resolve(&self.root)?;
            let result = (|| {
                if let Some(parent) = host.parent() {
                    fs::create_dir_all(parent)?;
                }
                replace(&backup, &host)
            })();
            if let Err(error) = result {
                return Err(fail(self, &relative, error));
            }
        }
        Ok(())
    }

    fn remove_recorded_absents(
        &mut self,
        fail: &dyn Fn(&mut Self, &RelativePath, io::Error) -> TransactionError,
    ) -> Result<(), TransactionError> {
        let mut absent = self.absent.clone();
        // Deepest first, so a directory is only removed once whatever this
        // transaction created inside it is gone.
        absent.sort_by_key(|(_, path)| std::cmp::Reverse(path.components().count()));
        for (directory, relative) in absent {
            let host = match relative.resolve(&self.root) {
                Ok(host) => host,
                Err(error) => return Err(fail(self, &relative, path_error(error))),
            };
            let result = match fs::symlink_metadata(&host) {
                Ok(metadata) if metadata.is_dir() && directory => fs::remove_dir(&host),
                Ok(metadata) if metadata.is_file() && !directory => remove_file(&host),
                _ => Ok(()),
            };
            if let Err(error) = result {
                return Err(fail(self, &relative, error));
            }
        }
        Ok(())
    }
}

use super::filesystem::{remove_file, replace};
use super::workspace::walk;
