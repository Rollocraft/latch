use std::{
    ffi::OsString,
    fs::File,
    io::{self, Read},
};

pub(crate) fn bounded_read(path: &OsString, maximum: usize) -> Result<Vec<u8>, String> {
    let read = || -> io::Result<Vec<u8>> {
        let file = File::open(path)?;
        if !file.metadata()?.is_file() {
            return Err(io::Error::other("expected a regular file"));
        }
        let mut bytes = Vec::new();
        file.take(maximum as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > maximum {
            return Err(io::Error::other(format!(
                "file exceeds {maximum} byte limit"
            )));
        }
        Ok(bytes)
    };
    read().map_err(|e| format!("cannot read {path:?}: {e}"))
}
