use crate::EventResult;
use latch_core::RiskScore;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditFilters {
    pub session: Option<String>,
    pub agent: Option<String>,
    pub owner: Option<String>,
    pub environment: Option<String>,
    pub result: Option<EventResult>,
    pub minimum_risk: Option<RiskScore>,
    pub maximum_risk: Option<RiskScore>,
    pub from_inclusive: Option<u64>,
    pub until_exclusive: Option<u64>,
}
