use super::validate_attributes::validate_attribute;
use super::{AuthorizationField, AuthorizationValidationError, Role};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedPrincipal {
    tenant: String,
    subject: String,
    roles: BTreeSet<Role>,
    not_before: u64,
    expires_at: u64,
}

impl AuthenticatedPrincipal {
    pub fn from_trusted_identity(
        trusted_tenant: impl Into<String>,
        trusted_subject: impl Into<String>,
        trusted_roles: impl IntoIterator<Item = Role>,
        not_before: u64,
        expires_at: u64,
    ) -> Result<Self, AuthorizationValidationError> {
        let tenant = trusted_tenant.into();
        validate_attribute(&tenant, AuthorizationField::Tenant)?;
        let subject = trusted_subject.into();
        validate_attribute(&subject, AuthorizationField::Subject)?;
        if not_before >= expires_at {
            return Err(AuthorizationValidationError::InvalidValidityWindow);
        }
        Ok(Self {
            tenant,
            subject,
            roles: trusted_roles.into_iter().collect(),
            not_before,
            expires_at,
        })
    }

    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn roles(&self) -> &BTreeSet<Role> {
        &self.roles
    }

    pub fn not_before(&self) -> u64 {
        self.not_before
    }

    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }
}
