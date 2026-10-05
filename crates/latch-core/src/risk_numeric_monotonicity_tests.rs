use super::risk_test_support::*;
use super::*;

#[test]
fn numeric_risk_factors_are_monotonic_across_their_scales() {
    let percentages: Vec<u8> = (0..=100).collect();
    assert_monotonic(&percentages, |f, v| f.baseline = RiskScore::new(v));
    assert_monotonic(&percentages, |f, v| f.trust = Trust::new(100 - v));
    assert_monotonic(&percentages, |f, v| {
        f.historical_anomaly = HistoricalAnomaly::new(v)
    });
    assert_monotonic(&percentages, |f, v| f.blast_radius = BlastRadius::new(v));
    assert_monotonic(
        &[
            0,
            1,
            1_023,
            1_024,
            1_025,
            1_048_575,
            1_048_576,
            1_048_577,
            1_073_741_823,
            1_073_741_824,
            1_073_741_825,
            u64::MAX - 1,
            u64::MAX,
        ],
        |f, v| f.data_volume_bytes = Some(v),
    );
    assert_monotonic(
        &[
            0,
            1,
            9_999,
            10_000,
            10_001,
            999_999,
            1_000_000,
            1_000_001,
            99_999_999,
            100_000_000,
            100_000_001,
            u64::MAX - 1,
            u64::MAX,
        ],
        |f, v| f.financial_amount = Some(FinancialAmount::new(v)),
    );
}
