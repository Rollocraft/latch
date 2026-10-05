use super::*;

#[test]
fn volume_and_financial_thresholds_are_stable_without_overflow() {
    for (bytes, points) in [
        (0, 0),
        (1, 2),
        (1_024, 2),
        (1_025, 10),
        (1_048_576, 10),
        (1_048_577, 20),
        (1_073_741_824, 20),
        (1_073_741_825, 30),
        (u64::MAX, 30),
    ] {
        assert_eq!(data_volume_points(bytes), points);
    }
    for (amount, points) in [
        (0, 0),
        (1, 2),
        (10_000, 2),
        (10_001, 15),
        (1_000_000, 15),
        (1_000_001, 25),
        (100_000_000, 25),
        (100_000_001, 35),
        (u64::MAX, 35),
    ] {
        assert_eq!(financial_amount_points(amount), points);
    }
}
