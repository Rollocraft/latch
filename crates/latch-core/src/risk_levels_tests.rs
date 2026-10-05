use super::risk_test_support::*;
use super::*;

#[test]
fn levels_are_stable_for_every_valid_score() {
    for value in 0..=100 {
        let expected = match value {
            0..=9 => RiskLevel::Minimal,
            10..=29 => RiskLevel::Low,
            30..=59 => RiskLevel::Moderate,
            60..=84 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };
        assert_eq!(minimal(value).assess().level(), expected);
    }
}
