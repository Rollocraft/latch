//! Explainable risk scoring with conservative unknown-factor defaults.

#[path = "risk_assessment.rs"]
mod risk_assessment;
#[path = "risk_calculation.rs"]
mod risk_calculation;
#[path = "risk_category_points.rs"]
mod risk_category_points;
#[path = "risk_contribution.rs"]
mod risk_contribution;
#[path = "risk_environment.rs"]
mod risk_environment;
#[path = "risk_factors.rs"]
mod risk_factors;
#[path = "risk_level.rs"]
mod risk_level;
#[path = "risk_numeric_points.rs"]
mod risk_numeric_points;
#[path = "risk_percentages.rs"]
mod risk_percentages;

pub use risk_assessment::{RiskAssessment, ScoreBreakdown};
pub use risk_calculation::calculate_risk;
pub use risk_contribution::{Contribution, FactorKind};
pub use risk_environment::{DestinationTrust, Environment, Sensitivity};
pub use risk_factors::{FinancialAmount, RiskFactors};
pub use risk_level::RiskLevel;
pub use risk_percentages::{BlastRadius, HistoricalAnomaly, Trust};

#[cfg(test)]
use crate::{Reversibility, RiskScore};
#[cfg(test)]
use risk_numeric_points::{data_volume_points, financial_amount_points};
#[cfg(test)]
#[path = "risk_action_independence_tests.rs"]
mod risk_action_independence_tests;
#[cfg(test)]
#[path = "risk_baseline_tests.rs"]
mod risk_baseline_tests;
#[cfg(test)]
#[path = "risk_breakdown_tests.rs"]
mod risk_breakdown_tests;
#[cfg(test)]
#[path = "risk_categorical_monotonicity_tests.rs"]
mod risk_categorical_monotonicity_tests;
#[cfg(test)]
#[path = "risk_levels_tests.rs"]
mod risk_levels_tests;
#[cfg(test)]
#[path = "risk_numeric_monotonicity_tests.rs"]
mod risk_numeric_monotonicity_tests;
#[cfg(test)]
#[path = "risk_percentage_bounds_tests.rs"]
mod risk_percentage_bounds_tests;
#[cfg(test)]
#[path = "risk_test_support.rs"]
mod risk_test_support;
#[cfg(test)]
#[path = "risk_thresholds_tests.rs"]
mod risk_thresholds_tests;
#[cfg(test)]
#[path = "risk_unknown_factors_tests.rs"]
mod risk_unknown_factors_tests;
