use super::error::{ABSENT, MARKER, MAXIMUM_ENTRIES, TransactionError, VERSION};
use crate::path::RelativePath;
use std::{fs, io, io::Write, path::Path};

pub(crate) fn overlaps(workspace: &Path, root: &Path) -> bool {
    workspace.starts_with(root) || root.starts_with(workspace)
}

pub(crate) fn write_marker(
    workspace: &Path,
    root: &Path,
    state_label: &str,
) -> Result<(), TransactionError> {
    let root = root.to_str().ok_or(TransactionError::InvalidWorkspace)?;
    let contents = format!("{VERSION}\n{state_label}\n{root}");
    let path = workspace.join(MARKER);
    let mut file = fs::File::create(&path)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

pub(crate) fn read_absent(workspace: &Path) -> Result<Vec<(bool, RelativePath)>, TransactionError> {
    let bytes = match fs::read(workspace.join(ABSENT)) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let mut records = Vec::new();
    let mut input = &bytes[..];
    while !input.is_empty() {
        if input.len() < 5 {
            return Err(TransactionError::InvalidWorkspace);
        }
        let directory = input[0] == 1;
        let length = u32::from_le_bytes(input[1..5].try_into().unwrap()) as usize;
        if input.len() < 5 + length {
            return Err(TransactionError::InvalidWorkspace);
        }
        let text = std::str::from_utf8(&input[5..5 + length])
            .map_err(|_| TransactionError::InvalidWorkspace)?;
        records.push((directory, RelativePath::new(text)?));
        input = &input[5 + length..];
    }
    Ok(records)
}

pub(crate) fn walk(root: &Path) -> Result<Vec<RelativePath>, TransactionError> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = match fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error.into()),
        };
        for entry in entries {
            let path = entry?.path();
            let metadata = fs::symlink_metadata(&path)?;
            if metadata.is_dir() {
                pending.push(path);
            } else {
                found.push(RelativePath::from_host(root, &path)?);
                if found.len() > MAXIMUM_ENTRIES {
                    return Err(TransactionError::Io(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "workspace exceeds entry limit",
                    )));
                }
            }
        }
    }
    found.sort();
    Ok(found)
}
