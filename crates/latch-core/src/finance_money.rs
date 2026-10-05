#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Currency([u8; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinanceError {
    InvalidCurrencyCode,
    CurrencyMismatch,
    InvalidThresholds,
    Overflow,
    Underflow,
}

impl Currency {
    pub fn new(code: &str) -> Result<Self, FinanceError> {
        let bytes: [u8; 3] = code
            .as_bytes()
            .try_into()
            .map_err(|_| FinanceError::InvalidCurrencyCode)?;
        if !bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(FinanceError::InvalidCurrencyCode);
        }
        Ok(Self(bytes))
    }

    pub fn code(&self) -> &str {
        std::str::from_utf8(&self.0).expect("validated ASCII currency code")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    currency: Currency,
    minor_units: u64,
}

impl Money {
    pub fn new(currency: Currency, minor_units: u64) -> Self {
        Self {
            currency,
            minor_units,
        }
    }

    pub fn currency(self) -> Currency {
        self.currency
    }
    pub fn minor_units(self) -> u64 {
        self.minor_units
    }

    pub fn checked_add(self, other: Self) -> Result<Self, FinanceError> {
        self.match_currency(other)?;
        let amount = self
            .minor_units
            .checked_add(other.minor_units)
            .ok_or(FinanceError::Overflow)?;
        Ok(Self::new(self.currency, amount))
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, FinanceError> {
        self.match_currency(other)?;
        let amount = self
            .minor_units
            .checked_sub(other.minor_units)
            .ok_or(FinanceError::Underflow)?;
        Ok(Self::new(self.currency, amount))
    }

    pub(super) fn match_currency(self, other: Self) -> Result<(), FinanceError> {
        if self.currency != other.currency {
            return Err(FinanceError::CurrencyMismatch);
        }
        Ok(())
    }
}
