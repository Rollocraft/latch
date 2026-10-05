use latch_substrate::{Clock, StdClock};

#[test]
fn std_clock_and_clone_share_monotonic_origin() {
    let clock = StdClock::new();
    let before = clock.monotonic_now();
    let cloned = clock.clone();
    let middle = cloned.monotonic_now();
    let after = clock.monotonic_now();
    assert!(before <= middle && middle <= after);
    let _ = clock.wall_time();
}
