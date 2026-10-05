use super::validate_attributes::validate_attribute;
use super::{AuthorizationField, AuthorizationValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceAttribute {
    Owner,
    Team,
    Environment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceAttributes {
    tenant: String,
    owner: Option<String>,
    pub(super) team: Option<String>,
    pub(super) environment: Option<String>,
}

impl ResourceAttributes {
    pub fn new(
        tenant: impl Into<String>,
        owner: Option<String>,
        team: Option<String>,
        environment: Option<String>,
    ) -> Result<Self, AuthorizationValidationError> {
        let tenant = tenant.into();
        validate_attribute(&tenant, AuthorizationField::Tenant)?;
        for (value, field) in [
            (&owner, AuthorizationField::Owner),
            (&team, AuthorizationField::Team),
            (&environment, AuthorizationField::Environment),
        ] {
            if let Some(value) = value {
                validate_attribute(value, field)?;
            }
        }
        Ok(Self {
            tenant,
            owner,
            team,
            environment,
        })
    }

    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    pub fn attribute(&self, attribute: ResourceAttribute) -> Option<&str> {
        match attribute {
            ResourceAttribute::Owner => self.owner.as_deref(),
            ResourceAttribute::Team => self.team.as_deref(),
            ResourceAttribute::Environment => self.environment.as_deref(),
        }
    }
}
