use super::{
    BlastRadius, DestinationTrust, Environment, HistoricalAnomaly, RiskAssessment, Sensitivity,
    Trust, calculate_risk,
};
use crate::{Reversibility, RiskScore};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FinancialAmount {
    minor_units: u64,
}

impl FinancialAmount {
    pub fn new(minor_units: u64) -> Self {
        Self { minor_units }
    }
    pub fn minor_units(self) -> u64 {
        self.minor_units
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RiskFactors {
    pub baseline: Option<RiskScore>,
    pub environment: Environment,
    pub sensitivity: Sensitivity,
    pub reversibility: Option<Reversibility>,
    pub data_volume_bytes: Option<u64>,
    pub trust: Option<Trust>,
    pub historical_anomaly: Option<HistoricalAnomaly>,
    pub destination_trust: DestinationTrust,
    pub financial_amount: Option<FinancialAmount>,
    pub blast_radius: Option<BlastRadius>,
}

impl RiskFactors {
    pub fn new(baseline: RiskScore, reversibility: Reversibility) -> Self {
        Self {
            baseline: Some(baseline),
            reversibility: Some(reversibility),
            ..Self::default()
        }
    }
    pub fn assess(&self) -> RiskAssessment {
        calculate_risk(self)
    }
}
