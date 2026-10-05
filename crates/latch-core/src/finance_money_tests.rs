use super::finance_test_support::*;
use super::*;

#[test]
fn currency_validates_shape_not_registry() {
    for code in ["EUR", "USD", "ZZZ"] {
        assert_eq!(Currency::new(code).unwrap().code(), code);
    }
    for code in [
        "", "EU", "EURO", "eur", "EuR", "E1R", " EUR", "EU\n", "€", "ÉUR",
    ] {
        assert_eq!(Currency::new(code), Err(FinanceError::InvalidCurrencyCode));
    }
}

#[test]
fn money_is_checked_and_currency_bound() {
    assert_eq!(eur(0).minor_units(), 0);
    assert_eq!(eur(u64::MAX).checked_add(eur(0)), Ok(eur(u64::MAX)));
    assert_eq!(
        eur(u64::MAX).checked_add(eur(1)),
        Err(FinanceError::Overflow)
    );
    assert_eq!(eur(0).checked_sub(eur(1)), Err(FinanceError::Underflow));
    assert_eq!(eur(u64::MAX).checked_sub(eur(u64::MAX)), Ok(eur(0)));
    assert_eq!(eur(4_999).checked_add(eur(1)), Ok(eur(5_000)));
    let usd = Money::new(Currency::new("USD").unwrap(), 0);
    assert_eq!(eur(0).checked_add(usd), Err(FinanceError::CurrencyMismatch));
    assert_eq!(eur(0).checked_sub(usd), Err(FinanceError::CurrencyMismatch));
}
