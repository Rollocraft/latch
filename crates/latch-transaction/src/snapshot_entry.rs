//! One recorded entry of a snapshot: what the path is, what it holds, and the
//! permission bits it carries on this platform.

use crate::path::PathError;
use latch_core::{Digest, Sha256};
use std::{
    fs::Metadata,
    io::{self, Read},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Directory,
    File {
        digest: Digest,
        bytes: u64,
    },
    /// The target is recorded exactly as stored, never resolved.
    Symlink {
        target: String,
    },
    /// Sockets, devices and anything else a transaction refuses to handle.
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: EntryKind,
    pub mode: Mode,
}

/// POSIX permission bits where the platform has them, otherwise the read-only
/// flag. The two are never compared against each other: a snapshot is only
/// meaningful against another snapshot of the same tree on the same host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mode(u32);

impl Mode {
    /// Free to construct: a mode is only ever compared with another mode read
    /// from the same platform, never interpreted across platforms.
    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub fn bits(self) -> u32 {
        self.0
    }
}

#[cfg(unix)]
pub(crate) fn mode_of(metadata: &Metadata) -> Mode {
    use std::os::unix::fs::PermissionsExt;
    Mode(metadata.permissions().mode() & 0o7777)
}

#[cfg(not(unix))]
pub(crate) fn mode_of(metadata: &Metadata) -> Mode {
    Mode(u32::from(metadata.permissions().readonly()))
}

/// Stream the file so that snapshotting a large tree cannot be turned into a
/// memory exhaustion primitive by an agent that writes one enormous file.
pub(crate) fn digest_of(path: &Path) -> io::Result<Digest> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            return Ok(hasher.finish());
        }
        hasher.update(&buffer[..read]);
    }
}

pub(crate) fn path_error(error: PathError) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("invalid path: {error:?}"),
    )
}
