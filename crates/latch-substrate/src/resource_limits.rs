use crate::{InvalidResourceLimit, Resource};
use std::{
    num::{NonZeroU32, NonZeroU64},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceLimits {
    cpu_time: Duration,
    memory_bytes: NonZeroU64,
    process_count: NonZeroU32,
    disk_bytes: NonZeroU64,
    runtime: Duration,
}

impl ResourceLimits {
    pub fn new(
        cpu_time: Duration,
        memory_bytes: u64,
        process_count: u32,
        disk_bytes: u64,
        runtime: Duration,
    ) -> Result<Self, InvalidResourceLimit> {
        if cpu_time.is_zero() {
            return Err(InvalidResourceLimit {
                resource: Resource::CpuTime,
            });
        }
        let memory_bytes = NonZeroU64::new(memory_bytes).ok_or(InvalidResourceLimit {
            resource: Resource::MemoryBytes,
        })?;
        let process_count = NonZeroU32::new(process_count).ok_or(InvalidResourceLimit {
            resource: Resource::ProcessCount,
        })?;
        let disk_bytes = NonZeroU64::new(disk_bytes).ok_or(InvalidResourceLimit {
            resource: Resource::DiskBytes,
        })?;
        if runtime.is_zero() {
            return Err(InvalidResourceLimit {
                resource: Resource::Runtime,
            });
        }
        Ok(Self {
            cpu_time,
            memory_bytes,
            process_count,
            disk_bytes,
            runtime,
        })
    }

    pub fn cpu_time(self) -> Duration {
        self.cpu_time
    }

    pub fn memory_bytes(self) -> u64 {
        self.memory_bytes.get()
    }

    pub fn process_count(self) -> u32 {
        self.process_count.get()
    }

    pub fn disk_bytes(self) -> u64 {
        self.disk_bytes.get()
    }

    pub fn runtime(self) -> Duration {
        self.runtime
    }
}
