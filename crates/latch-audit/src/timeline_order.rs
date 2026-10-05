use crate::{AuditEvent, EventResult};
use std::cmp::Ordering;

pub(super) fn timeline_order(left: &AuditEvent, right: &AuditEvent) -> Ordering {
    (
        left.timestamp,
        &left.session,
        &left.agent,
        &left.action_id,
        result_rank(left.result),
    )
        .cmp(&(
            right.timestamp,
            &right.session,
            &right.agent,
            &right.action_id,
            result_rank(right.result),
        ))
        .then_with(|| {
            (
                &left.organization,
                &left.owner,
                &left.environment,
                &left.action,
                left.risk,
                &left.resource,
                &left.policies,
            )
                .cmp(&(
                    &right.organization,
                    &right.owner,
                    &right.environment,
                    &right.action,
                    right.risk,
                    &right.resource,
                    &right.policies,
                ))
        })
}

fn result_rank(result: EventResult) -> u8 {
    match result {
        EventResult::ApprovalRequired => 0,
        EventResult::Denied => 1,
        EventResult::ReplayRejected => 2,
        EventResult::BudgetRejected => 3,
        EventResult::Authorized => 4,
        EventResult::PreparationFailed => 5,
        EventResult::Started => 6,
        EventResult::Succeeded => 7,
        EventResult::Failed => 8,
    }
}
