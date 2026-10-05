use super::risk_category_points::*;
use super::risk_numeric_points::*;
use super::{
    Contribution, DestinationTrust, Environment, FactorKind, RiskAssessment, RiskFactors,
    Sensitivity,
};
use crate::RiskScore;

pub fn calculate_risk(factors: &RiskFactors) -> RiskAssessment {
    let contribution = |factor, points, unknown| Contribution {
        factor,
        points,
        unknown,
    };
    RiskAssessment::from_contributions([
        contribution(
            FactorKind::Baseline,
            factors.baseline.map_or(100, RiskScore::value),
            factors.baseline.is_none(),
        ),
        contribution(
            FactorKind::Environment,
            environment_points(factors.environment),
            factors.environment == Environment::Unknown,
        ),
        contribution(
            FactorKind::Sensitivity,
            sensitivity_points(factors.sensitivity),
            factors.sensitivity == Sensitivity::Unknown,
        ),
        contribution(
            FactorKind::Reversibility,
            reversibility_points(factors.reversibility),
            factors.reversibility.is_none(),
        ),
        contribution(
            FactorKind::DataVolume,
            factors.data_volume_bytes.map_or(30, data_volume_points),
            factors.data_volume_bytes.is_none(),
        ),
        contribution(
            FactorKind::Trust,
            factors
                .trust
                .map_or(25, |trust| scaled_points(100 - trust.value(), 25)),
            factors.trust.is_none(),
        ),
        contribution(
            FactorKind::HistoricalAnomaly,
            factors
                .historical_anomaly
                .map_or(20, |anomaly| scaled_points(anomaly.value(), 20)),
            factors.historical_anomaly.is_none(),
        ),
        contribution(
            FactorKind::DestinationTrust,
            destination_points(factors.destination_trust),
            factors.destination_trust == DestinationTrust::Unknown,
        ),
        contribution(
            FactorKind::FinancialAmount,
            factors
                .financial_amount
                .map_or(35, |amount| financial_amount_points(amount.minor_units())),
            factors.financial_amount.is_none(),
        ),
        contribution(
            FactorKind::BlastRadius,
            factors
                .blast_radius
                .map_or(30, |radius| scaled_points(radius.value(), 30)),
            factors.blast_radius.is_none(),
        ),
    ])
}
