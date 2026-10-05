use crate::DomainName;
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    ExplicitAllow,
    ApprovalRequired,
    ExplicitDeny,
    DefaultDeny,
    /// The connection named an address the runtime never resolved, which is
    /// how a name-based rule would otherwise be walked around.
    UnresolvedAddress,
    /// The name has no live resolution, so what it points at is unknown.
    UnresolvedName,
    /// The name resolved into the host's own network. Blocked by default
    /// because this is what DNS rebinding aims at, including cloud metadata.
    PrivateAddress(IpAddr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub outcome: Outcome,
    pub reason: Reason,
    /// Every name considered, including the CNAME chain that led to the
    /// address. All of them must pass: a chain is only as allowed as its
    /// least allowed link.
    pub names: Vec<DomainName>,
    /// Every rule that applied, for explaining the answer.
    pub matched: Vec<String>,
    /// The rule that actually decided, if any did.
    pub decided_by: Option<String>,
}
