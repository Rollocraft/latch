use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    CpuTime,
    MemoryBytes,
    ProcessCount,
    DiskBytes,
    Runtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidResourceLimit {
    pub resource: Resource,
}

impl fmt::Display for InvalidResourceLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} limit must be nonzero", self.resource)
    }
}

impl Error for InvalidResourceLimit {}
