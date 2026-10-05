use super::query_validation::{validate_filters, validate_text};
use super::{AuditFilters, QueryError};
use crate::AuditEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditQuery {
    organization: String,
    filters: AuditFilters,
}

impl AuditQuery {
    pub fn new(organization: impl Into<String>, filters: AuditFilters) -> Result<Self, QueryError> {
        let organization = organization.into();
        validate_text("organization", &organization)?;
        validate_filters(&filters)?;
        Ok(Self {
            organization,
            filters,
        })
    }

    pub fn organization(&self) -> &str {
        &self.organization
    }

    pub fn filters(&self) -> &AuditFilters {
        &self.filters
    }

    pub(super) fn matches(&self, event: &AuditEvent) -> bool {
        let filters = &self.filters;
        event.organization == self.organization
            && filters.session.as_ref().is_none_or(|v| *v == event.session)
            && filters.agent.as_ref().is_none_or(|v| *v == event.agent)
            && filters.owner.as_ref().is_none_or(|v| *v == event.owner)
            && filters
                .environment
                .as_ref()
                .is_none_or(|v| *v == event.environment)
            && filters.result.is_none_or(|v| v == event.result)
            && filters.minimum_risk.is_none_or(|v| event.risk >= v)
            && filters.maximum_risk.is_none_or(|v| event.risk <= v)
            && filters.from_inclusive.is_none_or(|v| event.timestamp >= v)
            && filters.until_exclusive.is_none_or(|v| event.timestamp < v)
    }
}
