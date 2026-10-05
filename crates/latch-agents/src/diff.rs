use serde::Serialize;

use crate::{AgentManifest, ManifestError, RequestedCapability};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PermissionDiff {
    added: Vec<RequestedCapability>,
    removed: Vec<RequestedCapability>,
    expanded: Vec<RequestedCapability>,
}

impl PermissionDiff {
    pub fn between(
        previous: &AgentManifest,
        proposed: &AgentManifest,
    ) -> Result<Self, ManifestError> {
        if previous.agent().name() != proposed.agent().name()
            || previous.identity() != proposed.identity()
        {
            return Err(ManifestError::DifferentAgent);
        }
        let added: Vec<_> = proposed
            .requires()
            .difference(previous.requires())
            .cloned()
            .collect();
        let removed = previous
            .requires()
            .difference(proposed.requires())
            .cloned()
            .collect();
        let expanded = added
            .iter()
            .filter(|candidate| {
                !previous
                    .requires()
                    .iter()
                    .any(|baseline| baseline.covers(candidate))
            })
            .cloned()
            .collect();
        Ok(Self {
            added,
            removed,
            expanded,
        })
    }

    pub fn initial(proposed: &AgentManifest) -> Self {
        let added: Vec<_> = proposed.requires().iter().cloned().collect();
        Self {
            expanded: added.clone(),
            added,
            removed: Vec::new(),
        }
    }

    pub fn added(&self) -> &[RequestedCapability] {
        &self.added
    }
    pub fn removed(&self) -> &[RequestedCapability] {
        &self.removed
    }
    pub fn expanded(&self) -> &[RequestedCapability] {
        &self.expanded
    }
    pub fn requires_review(&self) -> bool {
        !self.expanded.is_empty()
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}
