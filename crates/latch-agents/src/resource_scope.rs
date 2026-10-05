use crate::ResourceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    Exact,
    Subtree,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceScope {
    kind: ScopeKind,
    resource: ResourceId,
}

impl ResourceScope {
    pub fn exact(resource: ResourceId) -> Self {
        Self {
            kind: ScopeKind::Exact,
            resource,
        }
    }
    pub fn subtree(resource: ResourceId) -> Self {
        Self {
            kind: ScopeKind::Subtree,
            resource,
        }
    }
    pub fn kind(&self) -> ScopeKind {
        self.kind
    }
    pub fn resource(&self) -> &ResourceId {
        &self.resource
    }

    pub fn contains(&self, resource: &ResourceId) -> bool {
        match self.kind {
            ScopeKind::Exact => self.resource == *resource,
            ScopeKind::Subtree => resource.within(&self.resource),
        }
    }

    pub fn covers(&self, other: &Self) -> bool {
        match self.kind {
            ScopeKind::Exact => self == other,
            ScopeKind::Subtree => self.contains(&other.resource),
        }
    }
}
