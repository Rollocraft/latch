use super::risk_test_support::*;
use super::*;

#[test]
fn unknown_factors_are_explicit_and_never_optimistic() {
    let unknown = RiskFactors::default();
    let worst = RiskFactors {
        baseline: RiskScore::new(100),
        environment: Environment::Production,
        sensitivity: Sensitivity::Restricted,
        reversibility: Some(Reversibility::Irreversible),
        data_volume_bytes: Some(u64::MAX),
        trust: Trust::new(0),
        historical_anomaly: HistoricalAnomaly::new(100),
        destination_trust: DestinationTrust::Untrusted,
        financial_amount: Some(FinancialAmount::new(u64::MAX)),
        blast_radius: BlastRadius::new(100),
    };
    for (missing, known) in unknown
        .assess()
        .breakdown()
        .contributions()
        .iter()
        .zip(worst.assess().breakdown().contributions())
    {
        assert!(missing.is_unknown());
        assert!(!known.is_unknown());
        assert!(missing.points() >= known.points());
    }
    assert_eq!(unknown.assess().score().value(), 100);
    assert_eq!(unknown.assess().level(), RiskLevel::Critical);
    assert_eq!(worst.assess().breakdown().total_points(), 395);
    assert_eq!(worst.assess().breakdown().capped_points(), 295);
    assert_eq!(FinancialAmount::new(u64::MAX).minor_units(), u64::MAX);
    assert_reconciles(&unknown);
    assert_reconciles(&worst);
    let partial = RiskFactors::new(RiskScore::new(5).unwrap(), Reversibility::FullyReversible);
    assert_eq!(
        partial
            .assess()
            .breakdown()
            .contributions()
            .iter()
            .filter(|c| c.is_unknown())
            .count(),
        8
    );
}
