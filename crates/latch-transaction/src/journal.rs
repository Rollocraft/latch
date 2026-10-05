use super::{Transaction, TransactionError};
use crate::path::RelativePath;
use std::{fs, io::Write, path::Path};

use super::error::ABSENT;
use super::filesystem::{copy_preserving, remove_file};

impl Transaction {
    /// Copy staged content over the host path, journaling what was there.
    pub(crate) fn publish(&mut self, path: &RelativePath) -> Result<(), TransactionError> {
        let host = path.resolve(&self.root)?;
        self.journal(path, &host)?;
        self.create_parents(path)?;
        replace(&self.area(super::STAGE, path)?, &host)?;
        Ok(())
    }

    pub(crate) fn erase(&mut self, path: &RelativePath) -> Result<(), TransactionError> {
        let host = path.resolve(&self.root)?;
        self.journal(path, &host)?;
        if host.exists() {
            remove_file(&host)?;
        }
        Ok(())
    }

    /// Record the host state a change is about to replace: either a copy of the
    /// file, or a note that nothing was there.
    fn journal(&mut self, path: &RelativePath, host: &Path) -> Result<(), TransactionError> {
        let backup = self.area(super::BACKUP, path)?;
        if backup.exists() || self.absent.contains(&(false, path.clone())) {
            return Ok(());
        }
        if host.is_file() {
            if let Some(parent) = backup.parent() {
                fs::create_dir_all(parent)?;
            }
            copy_preserving(host, &backup)?;
        } else {
            self.record_absent(false, path)?;
        }
        Ok(())
    }

    /// Create missing parent directories, recording each one so that a rollback
    /// leaves no empty directory behind.
    fn create_parents(&mut self, path: &RelativePath) -> Result<(), TransactionError> {
        let components: Vec<&str> = path.components().collect();
        let mut prefix = String::new();
        for component in &components[..components.len() - 1] {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(component);
            let relative = RelativePath::new(&prefix)?;
            let directory = relative.resolve(&self.root)?;
            if !directory.exists() {
                self.record_absent(true, &relative)?;
                fs::create_dir(&directory)?;
            }
        }
        Ok(())
    }

    pub(crate) fn record_absent(
        &mut self,
        directory: bool,
        path: &RelativePath,
    ) -> Result<(), TransactionError> {
        let mut record = vec![u8::from(directory)];
        let bytes = path.as_str().as_bytes();
        record.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        record.extend_from_slice(bytes);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.workspace.join(ABSENT))?;
        file.write_all(&record)?;
        file.sync_all()?;
        self.absent.push((directory, path.clone()));
        Ok(())
    }
}

use super::filesystem::replace;
