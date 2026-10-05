use super::{BudgetError, ReservationState, UsageAmount, UsageDimension};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetBalance {
    pub(super) limit: UsageAmount,
    pub(super) reserved: u64,
    pub(super) consumed: u64,
}

impl BudgetBalance {
    pub fn limit(self) -> UsageAmount {
        self.limit
    }
    pub fn reserved(self) -> u64 {
        self.reserved
    }
    pub fn consumed(self) -> u64 {
        self.consumed
    }

    pub fn available(self) -> Result<u64, BudgetError> {
        self.limit
            .units()
            .checked_sub(self.consumed)
            .and_then(|value| value.checked_sub(self.reserved))
            .ok_or(BudgetError::ArithmeticOverflow)
    }

    pub(super) fn reserve(
        &mut self,
        dimension: UsageDimension,
        amount: UsageAmount,
    ) -> Result<(), BudgetError> {
        self.limit.match_unit(amount, dimension)?;
        let reserved = self
            .reserved
            .checked_add(amount.units())
            .ok_or(BudgetError::ArithmeticOverflow)?;
        let total = self
            .consumed
            .checked_add(reserved)
            .ok_or(BudgetError::ArithmeticOverflow)?;
        if total > self.limit.units() {
            return Err(BudgetError::LimitExceeded(dimension));
        }
        self.reserved = reserved;
        Ok(())
    }

    pub(super) fn finish(
        &mut self,
        amount: UsageAmount,
        state: ReservationState,
    ) -> Result<(), BudgetError> {
        self.reserved = self
            .reserved
            .checked_sub(amount.units())
            .ok_or(BudgetError::ArithmeticOverflow)?;
        if state != ReservationState::CancelledBeforeExecution {
            self.consumed = self
                .consumed
                .checked_add(amount.units())
                .ok_or(BudgetError::ArithmeticOverflow)?;
        }
        Ok(())
    }
}
