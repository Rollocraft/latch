use super::audit_frame_bytes::string;
use crate::{AuditEvent, EventResult};
use std::io;

pub(super) fn encode(event: &AuditEvent) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&event.timestamp.to_le_bytes());
    for value in [
        &event.organization,
        &event.owner,
        &event.agent,
        &event.session,
        &event.action_id,
        &event.action,
        &event.resource,
        &event.environment,
    ] {
        string(&mut buffer, value)?;
    }
    buffer.push(event.risk.value());
    buffer.push(match event.result {
        EventResult::Denied => 0,
        EventResult::ApprovalRequired => 1,
        EventResult::Authorized => 2,
        EventResult::Started => 3,
        EventResult::Succeeded => 4,
        EventResult::Failed => 5,
        EventResult::ReplayRejected => 6,
        EventResult::BudgetRejected => 7,
        EventResult::PreparationFailed => 8,
    });
    let count = u32::try_from(event.policies.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "too many policies"))?;
    buffer.extend_from_slice(&count.to_le_bytes());
    for policy in &event.policies {
        string(&mut buffer, policy)?;
    }
    Ok(buffer)
}
