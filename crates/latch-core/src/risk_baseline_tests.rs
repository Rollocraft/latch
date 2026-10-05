use super::risk_test_support::*;
use super::*;

#[test]
fn high_trust_preserves_every_baseline_and_other_risk() {
    for baseline in 0..=100 {
        let mut factors = minimal(baseline);
        assert_eq!(factors.assess().score().value(), baseline);
        factors.environment = Environment::Production;
        assert_eq!(
            factors.assess().score().value(),
            baseline.saturating_add(45).min(100)
        );
        assert_reconciles(&factors);
    }
}
