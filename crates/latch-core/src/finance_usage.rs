use super::{BudgetError, Money};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsageDimension {
    ApiTokens,
    CloudSpend,
    GpuMilliseconds,
    StorageBytes,
    Requests,
    Emails,
    Refunds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageAmount {
    Units(u64),
    Money(Money),
}

impl UsageAmount {
    pub fn units(self) -> u64 {
        match self {
            Self::Units(value) => value,
            Self::Money(value) => value.minor_units(),
        }
    }
    pub(super) fn validate(self, dimension: UsageDimension) -> Result<(), BudgetError> {
        let monetary = matches!(
            dimension,
            UsageDimension::CloudSpend | UsageDimension::Refunds
        );
        if monetary != matches!(self, Self::Money(_)) {
            return Err(BudgetError::UnitMismatch(dimension));
        }
        Ok(())
    }
    pub(super) fn match_unit(
        self,
        other: Self,
        dimension: UsageDimension,
    ) -> Result<(), BudgetError> {
        self.validate(dimension)?;
        other.validate(dimension)?;
        if let (Self::Money(left), Self::Money(right)) = (self, other)
            && left.currency() != right.currency()
        {
            return Err(BudgetError::CurrencyMismatch(dimension));
        }
        Ok(())
    }
}
