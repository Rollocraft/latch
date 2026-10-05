use latch_transaction::{MAXIMUM_ENTRIES, Snapshot, Transaction};
use std::path::{Path, PathBuf};

pub struct Fixture {
    pub(crate) base: PathBuf,
}

impl Fixture {
    pub fn new(label: &str) -> Self {
        let base = std::env::temp_dir().join(format!(
            "latch-tx-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(base.join("root")).unwrap();
        let fixture = Self { base };
        fixture.write("src/main.rs", "fn main() {}");
        fixture.write("src/lib.rs", "// lib");
        fixture.write("docs/readme.md", "# readme");
        fixture.write("keep.txt", "keep");
        fixture
    }

    pub fn root(&self) -> PathBuf {
        self.base.join("root")
    }
    pub fn workspace(&self) -> PathBuf {
        self.base.join("workspace")
    }
    pub fn host(&self, relative: &str) -> PathBuf {
        self.root().join(relative)
    }

    pub fn write(&self, relative: &str, contents: &str) {
        let path = self.host(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }

    pub fn read(&self, relative: &str) -> String {
        std::fs::read_to_string(self.host(relative)).unwrap()
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot::of(self.root(), MAXIMUM_ENTRIES).unwrap()
    }

    pub fn begin(&self) -> Transaction {
        Transaction::begin(self.root(), self.workspace()).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Read-only files are part of the tests, and Windows refuses to delete
        // them, so clear the flag before removing the tree. Making a file in
        // this private temporary tree writable for everyone is harmless.
        #[allow(clippy::permissions_set_readonly_false)]
        fn clear(path: &Path) {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Ok(metadata) = std::fs::symlink_metadata(&path) {
                        let mut permissions = metadata.permissions();
                        if permissions.readonly() {
                            permissions.set_readonly(false);
                            let _ = std::fs::set_permissions(&path, permissions);
                        }
                        if metadata.is_dir() && !metadata.file_type().is_symlink() {
                            clear(&path);
                        }
                    }
                }
            }
        }
        clear(&self.base);
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

pub fn stage_all(transaction: &mut Transaction) {
    transaction
        .write("src/main.rs", b"fn main() { changed() }")
        .unwrap();
    transaction
        .write("generated/deep/file.txt", b"new file")
        .unwrap();
    transaction.remove("keep.txt").unwrap();
    transaction
        .rename("docs/readme.md", "docs/manual.md")
        .unwrap();
    // Rewriting identical content is not a change to anything.
    transaction.write("src/lib.rs", b"// lib").unwrap();
}
