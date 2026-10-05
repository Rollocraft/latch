use super::{BudgetError, MAX_USAGE_DIMENSIONS, UsageAmount, UsageDimension};
use std::collections::BTreeMap;

pub(super) fn collect_dimensions(
    values: &[(UsageDimension, UsageAmount)],
) -> Result<BTreeMap<UsageDimension, UsageAmount>, BudgetError> {
    if values.is_empty() || values.len() > MAX_USAGE_DIMENSIONS {
        return Err(BudgetError::InvalidDimensionCount);
    }
    let mut result = BTreeMap::new();
    for &(dimension, amount) in values {
        amount.validate(dimension)?;
        if result.insert(dimension, amount).is_some() {
            return Err(BudgetError::DuplicateDimension(dimension));
        }
    }
    Ok(result)
}
