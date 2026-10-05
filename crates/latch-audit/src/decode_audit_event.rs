use super::audit_frame_bytes::*;
use crate::{AuditEvent, EventResult};
use std::io;

pub(super) fn decode(mut frame: &[u8]) -> io::Result<AuditEvent> {
    let timestamp = u64::from_le_bytes(take(&mut frame, 8)?.try_into().unwrap());
    let organization = text(&mut frame)?;
    let owner = text(&mut frame)?;
    let agent = text(&mut frame)?;
    let session = text(&mut frame)?;
    let action_id = text(&mut frame)?;
    let action = text(&mut frame)?;
    let resource = text(&mut frame)?;
    let environment = text(&mut frame)?;
    let risk = latch_core::RiskScore::new(take(&mut frame, 1)?[0])
        .ok_or_else(|| invalid("invalid risk"))?;
    let result = match take(&mut frame, 1)?[0] {
        0 => EventResult::Denied,
        1 => EventResult::ApprovalRequired,
        2 => EventResult::Authorized,
        3 => EventResult::Started,
        4 => EventResult::Succeeded,
        5 => EventResult::Failed,
        6 => EventResult::ReplayRejected,
        7 => EventResult::BudgetRejected,
        8 => EventResult::PreparationFailed,
        _ => return Err(invalid("unknown event result")),
    };
    let count = number(&mut frame)? as usize;
    if count > frame.len() / 4 {
        return Err(invalid("invalid policy count"));
    }
    let mut policies = Vec::new();
    for _ in 0..count {
        policies.push(text(&mut frame)?);
    }
    if !frame.is_empty() {
        return Err(invalid("trailing event data"));
    }
    Ok(AuditEvent {
        timestamp,
        organization,
        owner,
        agent,
        session,
        action_id,
        action,
        resource,
        environment,
        risk,
        result,
        policies,
    })
}
