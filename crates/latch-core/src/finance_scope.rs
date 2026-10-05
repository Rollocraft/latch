use super::{BudgetError, MAX_SCOPE_BYTES};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BudgetScope {
    tenant: String,
    session: String,
}

impl BudgetScope {
    pub fn new(tenant: &str, session: &str) -> Result<Self, BudgetError> {
        validate_identifier(tenant)?;
        validate_identifier(session)?;
        Ok(Self {
            tenant: tenant.into(),
            session: session.into(),
        })
    }
    pub fn tenant(&self) -> &str {
        &self.tenant
    }
    pub fn session(&self) -> &str {
        &self.session
    }
}

pub(super) fn validate_identifier(value: &str) -> Result<(), BudgetError> {
    if value.is_empty()
        || value.len() > MAX_SCOPE_BYTES
        || value.bytes().any(|b| !b.is_ascii_graphic())
    {
        return Err(BudgetError::InvalidIdentifier);
    }
    Ok(())
}
