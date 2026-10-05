use crate::{Reversibility, RiskScore};

/// Arguments remain separate; adapters must not implicitly interpret shell code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub id: String,
    pub actor: String,
    pub session_id: String,
    pub name: String,
    pub resource: String,
    pub environment: String,
    pub arguments: Vec<String>,
    pub reversibility: Reversibility,
    pub risk: RiskScore,
}
