/// A risk score on the inclusive 0 to 100 scale required by the action model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RiskScore(u8);

impl RiskScore {
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }
    pub fn value(self) -> u8 {
        self.0
    }
}
