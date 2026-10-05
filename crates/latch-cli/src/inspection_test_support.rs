use std::{ffi::OsString, path::PathBuf};

pub struct Fixture(pub PathBuf);

impl Fixture {
    pub fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "latch-inspection-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn invoke(args: Vec<OsString>) -> Result<String, String> {
    let mut output = Vec::new();
    super::run(args, &mut output)?;
    Ok(String::from_utf8(output).unwrap())
}
