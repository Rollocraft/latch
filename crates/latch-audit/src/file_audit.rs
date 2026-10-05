use super::{audit_frame_bytes::HEADER, scan_audit_segment::scan};
use crate::AuditEvent;
use latch_core::{Digest, sha256};
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

/// An append-only audit segment, owned by one writer. Existing paths are never
/// truncated. Each append is synchronized before success is returned. A write
/// failure permanently poisons this writer because the last frame may be
/// incomplete.
///
/// Every frame carries the hash of the whole segment up to and including
/// itself, so editing, reordering, dropping or inserting an event invalidates
/// the chain from that point on. This makes tampering evident to a reader; it
/// does not prevent it. Someone who can rewrite the file can recompute the
/// chain, so the head must be published to an authenticated checkpoint outside
/// this file for the guarantee to hold. The containing directory must be
/// controlled by the runtime.
pub struct FileAudit {
    pub(super) file: File,
    pub(super) head: Digest,
    pub(super) failed: bool,
}

impl FileAudit {
    /// Read and verify a complete segment. The caller selects a total byte limit
    /// to bound memory use. A truncated final frame is an error, never silently
    /// discarded: a rejected segment must be treated as an integrity incident
    /// rather than repaired by dropping the unreadable remainder.
    pub fn read(path: impl AsRef<Path>, maximum_bytes: usize) -> io::Result<Vec<AuditEvent>> {
        Ok(scan(path, maximum_bytes)?.0)
    }

    /// Verify a segment without retaining its events and return the chain head,
    /// which the caller compares against an independently stored checkpoint.
    pub fn verify(path: impl AsRef<Path>, maximum_bytes: usize) -> io::Result<Digest> {
        Ok(scan(path, maximum_bytes)?.1)
    }

    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
        file.write_all(HEADER)?;
        file.sync_all()?;
        Ok(Self {
            file,
            head: sha256(HEADER),
            failed: false,
        })
    }

    /// Continue an existing segment after a restart. The whole chain is verified
    /// first, so a writer never extends a segment that has already been tampered
    /// with or left incomplete.
    pub fn resume(path: impl AsRef<Path>, maximum_bytes: usize) -> io::Result<Self> {
        let head = Self::verify(&path, maximum_bytes)?;
        Ok(Self {
            file: OpenOptions::new().append(true).open(path)?,
            head,
            failed: false,
        })
    }

    /// The hash covering every event written so far.
    pub fn head(&self) -> Digest {
        self.head
    }
}
