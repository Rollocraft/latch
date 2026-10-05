use crate::RiskScore;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Minimal,
    Low,
    Moderate,
    High,
    Critical,
}

impl RiskLevel {
    pub fn from_score(score: RiskScore) -> Self {
        match score.value() {
            0..=9 => Self::Minimal,
            10..=29 => Self::Low,
            30..=59 => Self::Moderate,
            60..=84 => Self::High,
            _ => Self::Critical,
        }
    }
}
