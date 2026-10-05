use super::*;

#[test]
fn validated_percentages_reject_every_out_of_range_value() {
    for value in 0..=u8::MAX {
        assert_eq!(Trust::new(value).is_some(), value <= 100);
        assert_eq!(HistoricalAnomaly::new(value).is_some(), value <= 100);
        assert_eq!(BlastRadius::new(value).is_some(), value <= 100);
        assert_eq!(RiskScore::new(value).is_some(), value <= 100);
        if value <= 100 {
            assert_eq!(Trust::new(value).unwrap().value(), value);
            assert_eq!(HistoricalAnomaly::new(value).unwrap().value(), value);
            assert_eq!(BlastRadius::new(value).unwrap().value(), value);
        }
    }
}
