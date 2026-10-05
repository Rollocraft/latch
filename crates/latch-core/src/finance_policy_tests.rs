use super::finance_test_support::*;
use super::*;

#[test]
fn refund_spec_has_inclusive_50_and_500_eur_approval_boundaries() {
    let policy = FinancialPolicy::new(eur(5_000), eur(50_000)).unwrap();
    for value in [0, 1, 2_900, 4_999] {
        assert_eq!(
            policy.decide(eur(value)),
            FinancialDecision {
                permission: FinancialPermission::Auto,
                reason: FinancialReason::BelowApprovalMinimum,
            }
        );
    }
    for value in [5_000, 5_001, 24_000, 49_999, 50_000] {
        assert_eq!(
            policy.decide(eur(value)),
            FinancialDecision {
                permission: FinancialPermission::ApprovalRequired,
                reason: FinancialReason::WithinInclusiveApprovalRange,
            }
        );
    }
    for value in [50_001, 240_000, u64::MAX] {
        assert_eq!(
            policy.decide(eur(value)),
            FinancialDecision {
                permission: FinancialPermission::Deny,
                reason: FinancialReason::AboveApprovalMaximum,
            }
        );
    }
    assert_eq!(
        policy.decide(Money::new(Currency::new("USD").unwrap(), 0)),
        FinancialDecision {
            permission: FinancialPermission::Deny,
            reason: FinancialReason::CurrencyMismatch,
        }
    );
}

#[test]
fn policy_rejects_invalid_configuration_and_handles_extreme_ranges() {
    assert_eq!(
        FinancialPolicy::new(eur(2), eur(1)),
        Err(FinanceError::InvalidThresholds)
    );
    assert_eq!(
        FinancialPolicy::new(eur(0), Money::new(Currency::new("USD").unwrap(), 1)),
        Err(FinanceError::CurrencyMismatch)
    );
    for value in [0, 5_000, u64::MAX] {
        let policy = FinancialPolicy::new(eur(value), eur(value)).unwrap();
        assert_eq!(
            policy.decide(eur(value)).permission,
            FinancialPermission::ApprovalRequired
        );
    }
    let policy = FinancialPolicy::new(eur(0), eur(u64::MAX)).unwrap();
    assert_eq!(
        policy.decide(eur(0)).permission,
        FinancialPermission::ApprovalRequired
    );
    assert_eq!(
        policy.decide(eur(u64::MAX)).permission,
        FinancialPermission::ApprovalRequired
    );
}
