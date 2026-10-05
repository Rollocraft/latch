use super::risk_test_support::*;
use super::*;

#[test]
fn breakdown_is_complete_ordered_and_deterministic() {
    let factors = RiskFactors {
        baseline: RiskScore::new(40),
        environment: Environment::Production,
        sensitivity: Sensitivity::Confidential,
        reversibility: Some(Reversibility::Irreversible),
        data_volume_bytes: Some(2_048),
        trust: Trust::new(50),
        historical_anomaly: HistoricalAnomaly::new(25),
        destination_trust: DestinationTrust::Known,
        financial_amount: Some(FinancialAmount::new(500_000)),
        blast_radius: BlastRadius::new(60),
    };
    let original = factors;
    let first = calculate_risk(&factors);
    for _ in 0..100 {
        assert_eq!(factors.assess(), first);
    }
    assert_eq!(factors, original);
    let expected = [
        (FactorKind::Baseline, 40),
        (FactorKind::Environment, 45),
        (FactorKind::Sensitivity, 25),
        (FactorKind::Reversibility, 35),
        (FactorKind::DataVolume, 10),
        (FactorKind::Trust, 12),
        (FactorKind::HistoricalAnomaly, 5),
        (FactorKind::DestinationTrust, 5),
        (FactorKind::FinancialAmount, 15),
        (FactorKind::BlastRadius, 18),
    ];
    for (actual, (factor, points)) in first.breakdown().contributions().iter().zip(expected) {
        assert_eq!(actual.factor(), factor);
        assert_eq!(actual.points(), points);
        assert!(!actual.is_unknown());
    }
    assert_eq!(first.breakdown().total_points(), 210);
    assert_eq!(first.breakdown().capped_points(), 110);
    assert_reconciles(&factors);
    assert_eq!(minimal(0).assess().breakdown().contributions().len(), 10);
    assert!(
        minimal(0)
            .assess()
            .breakdown()
            .contributions()
            .iter()
            .all(|c| c.points() == 0)
    );
}
