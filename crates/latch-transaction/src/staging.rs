use super::filesystem::{apply_mode, clear_read_only, mode_of, remove_file};
use super::{Transaction, TransactionError};
use crate::path::RelativePath;
use crate::snapshot::Mode;
use std::{fs, io::Write};

impl Transaction {
    pub fn read(&self, path: &str) -> Result<Vec<u8>, TransactionError> {
        let relative = RelativePath::new(path)?;
        let staged = self.area(STAGE, &relative)?;
        if staged.is_file() {
            return Ok(fs::read(staged)?);
        }
        if self.area(TOMB, &relative)?.exists() {
            return Err(TransactionError::NotFound(relative));
        }
        let host = relative.resolve(&self.root)?;
        match fs::symlink_metadata(&host) {
            Ok(metadata) if metadata.is_file() => Ok(fs::read(&host)?),
            Ok(_) => Err(TransactionError::NotAFile(relative)),
            Err(_) => Err(TransactionError::NotFound(relative)),
        }
    }

    pub fn write(&mut self, path: &str, contents: &[u8]) -> Result<(), TransactionError> {
        self.require_open()?;
        let relative = RelativePath::new(path)?;
        let host = relative.resolve(&self.root)?;
        let mode = match fs::symlink_metadata(&host) {
            Ok(metadata) if metadata.is_file() => Some(mode_of(&metadata)),
            Ok(_) => return Err(TransactionError::NotAFile(relative)),
            Err(_) => None,
        };
        let staged = self.area(STAGE, &relative)?;
        if let Some(parent) = staged.parent() {
            fs::create_dir_all(parent)?;
        }
        clear_read_only(&staged)?;
        let mut file = fs::File::create(&staged)?;
        file.write_all(contents)?;
        file.sync_all()?;
        // Mirror the host's permissions so that changing content alone is not
        // reported as a permission change as well.
        if let Some(mode) = mode {
            apply_mode(&staged, mode)?;
        }
        let tomb = self.area(TOMB, &relative)?;
        if tomb.exists() {
            remove_file(&tomb)?;
        }
        Ok(())
    }

    pub fn remove(&mut self, path: &str) -> Result<(), TransactionError> {
        self.require_open()?;
        let relative = RelativePath::new(path)?;
        let host = relative.resolve(&self.root)?;
        let staged = self.area(STAGE, &relative)?;
        match fs::symlink_metadata(&host) {
            Ok(metadata) if !metadata.is_file() => {
                return Err(TransactionError::NotAFile(relative));
            }
            Err(_) if !staged.is_file() => return Err(TransactionError::NotFound(relative)),
            _ => {}
        }
        if staged.is_file() {
            remove_file(&staged)?;
        }
        let tomb = self.area(TOMB, &relative)?;
        if let Some(parent) = tomb.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(&tomb)?.sync_all()?;
        Ok(())
    }

    /// Staged as a deletion plus a creation. `changes` reports the pair as one
    /// move when the content makes the pairing unambiguous.
    pub fn rename(&mut self, from: &str, to: &str) -> Result<(), TransactionError> {
        self.require_open()?;
        let contents = self.read(from)?;
        self.write(to, &contents)?;
        self.remove(from)
    }

    /// Stage a permission change. On platforms without POSIX modes the write
    /// bit is mapped to the read-only flag, which is the only bit they have.
    pub fn set_mode(&mut self, path: &str, mode: u32) -> Result<(), TransactionError> {
        self.require_open()?;
        let relative = RelativePath::new(path)?;
        let staged = self.area(STAGE, &relative)?;
        if !staged.is_file() {
            let contents = self.read(path)?;
            self.write(path, &contents)?;
        }
        apply_mode(&staged, Mode::from_bits(mode))?;
        Ok(())
    }
}

use super::{STAGE, TOMB};
