use crate::IdentityBinding;
use latch_core::AgentIdentity;

impl IdentityBinding {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn organization(&self) -> &str {
        &self.organization
    }
    pub fn team(&self) -> &str {
        &self.team
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn purpose(&self) -> &str {
        &self.purpose
    }
    pub fn runtime(&self) -> &str {
        &self.runtime
    }
    pub fn environment(&self) -> &str {
        &self.environment
    }
    pub fn device(&self) -> &str {
        &self.device
    }

    pub fn matches(&self, identity: &AgentIdentity) -> bool {
        self.id == identity.id
            && self.organization == identity.organization
            && self.team == identity.team
            && self.owner == identity.owner
            && self.purpose == identity.purpose
            && self.runtime == identity.runtime
            && self.environment == identity.environment
            && self.device == identity.device
    }
}
