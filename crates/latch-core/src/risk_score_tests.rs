use super::*;

#[test]
fn risk_bounds() {
    assert_eq!(RiskScore::new(0).unwrap().value(), 0);
    assert_eq!(RiskScore::new(100).unwrap().value(), 100);
    for value in 101..=u8::MAX {
        assert!(RiskScore::new(value).is_none());
    }
}
