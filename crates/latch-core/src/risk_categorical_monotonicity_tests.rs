use super::risk_test_support::*;
use super::*;

#[test]
fn categorical_risk_factors_are_monotonic() {
    assert_monotonic(
        &[
            Environment::Development,
            Environment::Staging,
            Environment::Production,
            Environment::Unknown,
        ],
        |f, v| f.environment = v,
    );
    assert_monotonic(
        &[
            Sensitivity::Public,
            Sensitivity::Internal,
            Sensitivity::Confidential,
            Sensitivity::Restricted,
            Sensitivity::Unknown,
        ],
        |f, v| f.sensitivity = v,
    );
    assert_monotonic(
        &[
            Some(Reversibility::FullyReversible),
            Some(Reversibility::PartiallyReversible),
            Some(Reversibility::Irreversible),
            None,
        ],
        |f, v| f.reversibility = v,
    );
    assert_monotonic(
        &[
            DestinationTrust::Trusted,
            DestinationTrust::Known,
            DestinationTrust::Unverified,
            DestinationTrust::Untrusted,
            DestinationTrust::Unknown,
        ],
        |f, v| f.destination_trust = v,
    );
}
