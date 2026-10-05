use super::*;

pub(super) fn minimal(baseline: u8) -> RiskFactors {
    RiskFactors {
        baseline: RiskScore::new(baseline),
        environment: Environment::Development,
        sensitivity: Sensitivity::Public,
        reversibility: Some(Reversibility::FullyReversible),
        data_volume_bytes: Some(0),
        trust: Trust::new(100),
        historical_anomaly: HistoricalAnomaly::new(0),
        destination_trust: DestinationTrust::Trusted,
        financial_amount: Some(FinancialAmount::new(0)),
        blast_radius: BlastRadius::new(0),
    }
}

pub(super) fn assert_reconciles(factors: &RiskFactors) {
    let assessment = factors.assess();
    let breakdown = assessment.breakdown();
    let sum: u16 = breakdown
        .contributions()
        .iter()
        .map(|c| u16::from(c.points()))
        .sum();
    assert_eq!(sum, breakdown.total_points());
    assert_eq!(
        sum - breakdown.capped_points(),
        u16::from(assessment.score().value())
    );
    assert!(assessment.score().value() <= 100);
    assert_eq!(
        assessment.level(),
        RiskLevel::from_score(assessment.score())
    );
}

pub(super) fn assert_monotonic<T: Copy>(values: &[T], update: impl Fn(&mut RiskFactors, T)) {
    for baseline in 0..=100 {
        let mut previous_score = 0;
        let mut previous_total = 0;
        for &value in values {
            let mut factors = minimal(baseline);
            update(&mut factors, value);
            let assessment = factors.assess();
            assert!(assessment.score().value() >= previous_score);
            assert!(assessment.breakdown().total_points() >= previous_total);
            previous_score = assessment.score().value();
            previous_total = assessment.breakdown().total_points();
            assert_reconciles(&factors);
        }
    }
}
