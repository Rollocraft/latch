use latch_substrate::{InvalidResourceLimit, Resource, ResourceLimits};
use std::time::Duration;

#[test]
fn every_resource_limit_must_be_nonzero() {
    let second = Duration::from_secs(1);
    let cases = [
        (Duration::ZERO, 1, 1, 1, second, Resource::CpuTime),
        (second, 0, 1, 1, second, Resource::MemoryBytes),
        (second, 1, 0, 1, second, Resource::ProcessCount),
        (second, 1, 1, 0, second, Resource::DiskBytes),
        (second, 1, 1, 1, Duration::ZERO, Resource::Runtime),
    ];
    for (cpu, memory, processes, disk, runtime, resource) in cases {
        assert_eq!(
            ResourceLimits::new(cpu, memory, processes, disk, runtime),
            Err(InvalidResourceLimit { resource })
        );
    }
}

#[test]
fn resource_limits_preserve_units_and_bounds() {
    let limits = ResourceLimits::new(
        Duration::from_nanos(1),
        u64::MAX,
        u32::MAX,
        1,
        Duration::MAX,
    )
    .unwrap();
    assert_eq!(limits.cpu_time(), Duration::from_nanos(1));
    assert_eq!(limits.memory_bytes(), u64::MAX);
    assert_eq!(limits.process_count(), u32::MAX);
    assert_eq!(limits.disk_bytes(), 1);
    assert_eq!(limits.runtime(), Duration::MAX);
}
