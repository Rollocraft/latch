use super::{FinanceError, Money};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialPermission {
    Auto,
    ApprovalRequired,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialReason {
    BelowApprovalMinimum,
    WithinInclusiveApprovalRange,
    AboveApprovalMaximum,
    CurrencyMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinancialDecision {
    pub permission: FinancialPermission,
    pub reason: FinancialReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinancialPolicy {
    approval_minimum_inclusive: Money,
    approval_maximum_inclusive: Money,
}

impl FinancialPolicy {
    pub fn new(minimum: Money, maximum: Money) -> Result<Self, FinanceError> {
        minimum.match_currency(maximum)?;
        if minimum.minor_units() > maximum.minor_units() {
            return Err(FinanceError::InvalidThresholds);
        }
        Ok(Self {
            approval_minimum_inclusive: minimum,
            approval_maximum_inclusive: maximum,
        })
    }

    pub fn decide(self, amount: Money) -> FinancialDecision {
        let (permission, reason) =
            if amount.currency() != self.approval_minimum_inclusive.currency() {
                (FinancialPermission::Deny, FinancialReason::CurrencyMismatch)
            } else if amount.minor_units() < self.approval_minimum_inclusive.minor_units() {
                (
                    FinancialPermission::Auto,
                    FinancialReason::BelowApprovalMinimum,
                )
            } else if amount.minor_units() <= self.approval_maximum_inclusive.minor_units() {
                (
                    FinancialPermission::ApprovalRequired,
                    FinancialReason::WithinInclusiveApprovalRange,
                )
            } else {
                (
                    FinancialPermission::Deny,
                    FinancialReason::AboveApprovalMaximum,
                )
            };
        FinancialDecision { permission, reason }
    }
}
