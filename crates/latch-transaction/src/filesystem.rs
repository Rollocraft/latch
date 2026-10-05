use crate::snapshot::Mode;
pub(crate) use crate::snapshot::mode_of;
use std::{
    fs, io,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Copy into place through a temporary file in the destination directory, so a
/// reader sees either the old file or the new one, never a half-written file.
pub(crate) fn replace(source: &Path, target: &Path) -> io::Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target has no directory"))?;
    let temporary = parent.join(format!(
        ".latch-{}-{}.tmp",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| {
        copy_preserving(source, &temporary)?;
        clear_read_only(target)?;
        fs::rename(&temporary, target)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// Copy a file with its permissions intact, durably. Flushing needs a writable
/// handle, but the copy inherits the source's permissions and may be read-only,
/// so it is made owner-writable first and the source mode is applied only after
/// the data is on disk.
pub(crate) fn copy_preserving(source: &Path, target: &Path) -> io::Result<()> {
    let mode = mode_of(&fs::symlink_metadata(source)?);
    fs::copy(source, target)?;
    apply_mode(target, Mode::from_bits(0o600))?;
    fs::OpenOptions::new()
        .write(true)
        .open(target)?
        .sync_all()?;
    apply_mode(target, mode)
}

pub(crate) fn remove_file(path: &Path) -> io::Result<()> {
    clear_read_only(path)?;
    fs::remove_file(path)
}

/// Windows refuses to replace or delete a read-only file; POSIX permissions do
/// not restrict the directory operation, so there is nothing to clear.
// The lint warns that clearing this flag makes a file world-writable on Unix,
// which is exactly why the code below never runs there.
#[cfg_attr(not(unix), allow(clippy::permissions_set_readonly_false))]
pub(crate) fn clear_read_only(path: &Path) -> io::Result<()> {
    #[cfg(not(unix))]
    if let Ok(metadata) = fs::symlink_metadata(path) {
        let mut permissions = metadata.permissions();
        if permissions.readonly() {
            permissions.set_readonly(false);
            fs::set_permissions(path, permissions)?;
        }
    }
    #[cfg(unix)]
    let _ = path;
    Ok(())
}

pub(crate) fn apply_mode(path: &Path, mode: Mode) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(mode.bits()))
    }
    #[cfg(not(unix))]
    {
        let mut permissions = fs::metadata(path)?.permissions();
        // Without POSIX modes the write bit is all that can be represented.
        permissions.set_readonly(mode.bits() & 0o200 == 0 && mode.bits() != 0);
        fs::set_permissions(path, permissions)
    }
}
