use super::{Contribution, RiskLevel};
use crate::RiskScore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreBreakdown {
    contributions: [Contribution; 10],
    total_points: u16,
    capped_points: u16,
}

impl ScoreBreakdown {
    pub fn contributions(&self) -> &[Contribution; 10] {
        &self.contributions
    }
    pub fn total_points(&self) -> u16 {
        self.total_points
    }
    pub fn capped_points(&self) -> u16 {
        self.capped_points
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskAssessment {
    score: RiskScore,
    level: RiskLevel,
    breakdown: ScoreBreakdown,
}

impl RiskAssessment {
    pub fn score(&self) -> RiskScore {
        self.score
    }
    pub fn level(&self) -> RiskLevel {
        self.level
    }
    pub fn breakdown(&self) -> &ScoreBreakdown {
        &self.breakdown
    }

    pub(super) fn from_contributions(contributions: [Contribution; 10]) -> Self {
        let total_points = contributions.iter().fold(0u16, |total, entry| {
            total.saturating_add(u16::from(entry.points()))
        });
        let score = RiskScore::new(total_points.min(100) as u8).expect("bounded risk score");
        Self {
            score,
            level: RiskLevel::from_score(score),
            breakdown: ScoreBreakdown {
                contributions,
                total_points,
                capped_points: total_points.saturating_sub(100),
            },
        }
    }
}
