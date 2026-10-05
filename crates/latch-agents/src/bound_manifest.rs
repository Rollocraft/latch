use crate::{ActionName, AgentManifest, ManifestError, ResourceId};
use latch_core::{Action, AgentIdentity, Session};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundManifest {
    manifest: AgentManifest,
    identity: AgentIdentity,
}

impl BoundManifest {
    pub(crate) fn new(
        manifest: AgentManifest,
        runtime_identity: &AgentIdentity,
        now: u64,
    ) -> Result<Self, ManifestError> {
        runtime_identity.validate_at(now)?;
        if !manifest.identity().matches(runtime_identity) {
            return Err(ManifestError::IdentityMismatch);
        }
        Ok(Self {
            manifest,
            identity: runtime_identity.clone(),
        })
    }

    pub fn manifest(&self) -> &AgentManifest {
        &self.manifest
    }
    pub fn identity(&self) -> &AgentIdentity {
        &self.identity
    }

    pub fn requests_action(
        &self,
        session: &Session,
        action: &Action,
        now: u64,
    ) -> Result<bool, ManifestError> {
        if session.identity() != &self.identity {
            return Err(ManifestError::IdentityMismatch);
        }
        session.validate_action(action, now)?;
        let name = ActionName::new(action.name.clone())?;
        let resource = ResourceId::new(action.resource.clone())?;
        Ok(self
            .manifest
            .requests(&name, &resource, &action.environment))
    }
}
