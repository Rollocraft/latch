use std::time::{Duration, Instant, SystemTime};

pub trait Clock {
    fn monotonic_now(&self) -> Duration;
    fn wall_time(&self) -> SystemTime;
}

#[derive(Debug, Clone)]
pub struct StdClock {
    origin: Instant,
}

impl StdClock {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Default for StdClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for StdClock {
    fn monotonic_now(&self) -> Duration {
        self.origin.elapsed()
    }

    fn wall_time(&self) -> SystemTime {
        SystemTime::now()
    }
}
