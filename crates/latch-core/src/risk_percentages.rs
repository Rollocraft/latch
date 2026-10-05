#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trust(u8);

impl Trust {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }
    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HistoricalAnomaly(u8);

impl HistoricalAnomaly {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }
    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlastRadius(u8);

impl BlastRadius {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }
    pub fn value(self) -> u8 {
        self.0
    }
}
