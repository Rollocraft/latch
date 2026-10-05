use super::{AuditFilters, MAX_QUERY_TEXT_BYTES, QueryError};

pub(super) fn validate_filters(filters: &AuditFilters) -> Result<(), QueryError> {
    for (name, value) in [
        ("session", &filters.session),
        ("agent", &filters.agent),
        ("owner", &filters.owner),
        ("environment", &filters.environment),
    ] {
        if let Some(value) = value {
            validate_text(name, value)?;
        }
    }
    if let (Some(start), Some(end)) = (filters.from_inclusive, filters.until_exclusive)
        && start >= end
    {
        return Err(QueryError::InvalidTimeWindow);
    }
    if let (Some(min), Some(max)) = (filters.minimum_risk, filters.maximum_risk)
        && min > max
    {
        return Err(QueryError::InvalidRiskRange);
    }
    Ok(())
}

pub(super) fn validate_text(field: &'static str, text: &str) -> Result<(), QueryError> {
    if text.trim().is_empty()
        || text.len() > MAX_QUERY_TEXT_BYTES
        || text.chars().any(char::is_control)
    {
        return Err(QueryError::InvalidText(field));
    }
    Ok(())
}
