//! Paths reaching a transaction are agent-supplied input, so they are validated
//! against one canonical form rather than normalized into one. Anything
//! ambiguous across the platforms the runtime targets is rejected instead of
//! being interpreted, because a path that means two things is an escape.

use std::path::{Path, PathBuf};

pub const MAXIMUM_PATH_BYTES: usize = 1024;
pub const MAXIMUM_COMPONENTS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathError {
    Empty,
    TooLong,
    TooDeep,
    NotRelative,
    Traversal,
    IllegalCharacter,
    ReservedName,
    /// A component of the resolved path is a symbolic link, which could point
    /// outside the root. Transactions never follow links to reach a file.
    Symlink,
}

/// A slash-separated path below a transaction root. Backslashes are rejected
/// rather than translated so that one path has exactly one spelling on every
/// platform, and so a Windows-style path cannot smuggle a separator past a
/// policy that matched on the string form.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelativePath(String);

const RESERVED: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

impl RelativePath {
    pub fn new(value: &str) -> Result<Self, PathError> {
        if value.is_empty() {
            return Err(PathError::Empty);
        }
        if value.len() > MAXIMUM_PATH_BYTES {
            return Err(PathError::TooLong);
        }
        if value.starts_with('/') {
            return Err(PathError::NotRelative);
        }
        if value.chars().any(|c| {
            c.is_control() || c == '\\' || c == ':' || c == '*' || c == '?' || c == '"' || c == '|'
        }) {
            return Err(PathError::IllegalCharacter);
        }
        let components: Vec<&str> = value.split('/').collect();
        if components.len() > MAXIMUM_COMPONENTS {
            return Err(PathError::TooDeep);
        }
        for component in &components {
            if component.is_empty() {
                return Err(PathError::Empty);
            }
            if *component == "." || *component == ".." {
                return Err(PathError::Traversal);
            }
            // Windows silently strips these, so two distinct names would address
            // one file; reserved device names address a device, never a file.
            if component.ends_with('.') || component.ends_with(' ') || component.starts_with(' ') {
                return Err(PathError::IllegalCharacter);
            }
            let stem = component.split('.').next().unwrap_or(component);
            if RESERVED.contains(&stem.to_ascii_lowercase().as_str()) {
                return Err(PathError::ReservedName);
            }
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.0.split('/')
    }

    pub fn parent(&self) -> Option<Self> {
        let (parent, _) = self.0.rsplit_once('/')?;
        Some(Self(parent.to_owned()))
    }

    /// Build the host path one component at a time, refusing to traverse a
    /// symbolic link. `root` must already be canonical: resolving the leaf with
    /// the platform's own canonicalization would follow exactly the links this
    /// check exists to reject.
    pub fn resolve(&self, root: &Path) -> Result<PathBuf, PathError> {
        let mut resolved = root.to_path_buf();
        let last = self.components().count() - 1;
        for (index, component) in self.components().enumerate() {
            resolved.push(component);
            match std::fs::symlink_metadata(&resolved) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    return Err(PathError::Symlink);
                }
                // A missing intermediate directory is not an escape: the caller
                // decides whether to create it or report the path as absent.
                Ok(metadata) if index < last && !metadata.is_dir() => {
                    return Err(PathError::NotRelative);
                }
                _ => {}
            }
        }
        Ok(resolved)
    }

    /// Recover the relative path of an entry discovered by walking a tree.
    pub(crate) fn from_host(root: &Path, path: &Path) -> Result<Self, PathError> {
        let suffix = path.strip_prefix(root).map_err(|_| PathError::Traversal)?;
        let mut value = String::new();
        for component in suffix.components() {
            let std::path::Component::Normal(part) = component else {
                return Err(PathError::Traversal);
            };
            if !value.is_empty() {
                value.push('/');
            }
            value.push_str(part.to_str().ok_or(PathError::IllegalCharacter)?);
        }
        Self::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ordinary_relative_paths() {
        for value in ["a", "src/main.rs", "a/b/c/d.txt", "file.name.with.dots"] {
            assert_eq!(RelativePath::new(value).unwrap().as_str(), value);
        }
        assert_eq!(
            RelativePath::new("a/b/c.txt").unwrap().parent(),
            Some(RelativePath::new("a/b").unwrap())
        );
        assert_eq!(RelativePath::new("a").unwrap().parent(), None);
    }

    #[test]
    fn rejects_every_form_of_escape() {
        for (value, expected) in [
            ("", PathError::Empty),
            ("/etc/passwd", PathError::NotRelative),
            ("a//b", PathError::Empty),
            ("..", PathError::Traversal),
            ("../secret", PathError::Traversal),
            ("a/../../secret", PathError::Traversal),
            ("a/./b", PathError::Traversal),
            ("a/..", PathError::Traversal),
            ("C:/windows", PathError::IllegalCharacter),
            ("a\\b", PathError::IllegalCharacter),
            ("a\nb", PathError::IllegalCharacter),
            ("a\0b", PathError::IllegalCharacter),
            ("file.txt:stream", PathError::IllegalCharacter),
            ("a/b.", PathError::IllegalCharacter),
            ("a/b ", PathError::IllegalCharacter),
            ("nul", PathError::ReservedName),
            ("a/CON.txt", PathError::ReservedName),
            ("a/lpt9", PathError::ReservedName),
        ] {
            assert_eq!(RelativePath::new(value), Err(expected), "{value:?}");
        }
        assert_eq!(
            RelativePath::new(&"a".repeat(MAXIMUM_PATH_BYTES + 1)),
            Err(PathError::TooLong)
        );
        assert_eq!(
            RelativePath::new(&vec!["a"; MAXIMUM_COMPONENTS + 1].join("/")),
            Err(PathError::TooDeep)
        );
        assert!(RelativePath::new(&vec!["a"; MAXIMUM_COMPONENTS].join("/")).is_ok());
    }
}
