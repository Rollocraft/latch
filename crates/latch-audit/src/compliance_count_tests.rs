use super::query_test_fixtures::*;
use crate::*;

#[test]
fn reports_count_events_not_implied_human_approvals_and_paginate_actions() {
    let mut events: Vec<_> = [
        EventResult::Denied,
        EventResult::ReplayRejected,
        EventResult::BudgetRejected,
        EventResult::ApprovalRequired,
        EventResult::Authorized,
        EventResult::Started,
        EventResult::Succeeded,
        EventResult::Failed,
        EventResult::PreparationFailed,
    ]
    .into_iter()
    .enumerate()
    .map(|(i, result)| {
        let mut e = event("acme", i as u64, result);
        e.risk = risk(if i < 3 { 69 } else { 70 });
        e.action = if i < 3 { "a.read" } else { "z.write" }.into();
        e
    })
    .collect();
    let mut foreign = event("other", 0, EventResult::Denied);
    foreign.action = "secret.action".into();
    events.push(foreign);
    let reader = AuditReader::new(&events);
    let q = query("acme");
    let report = reader.report(&q, page(0, 1), risk(70));
    assert_eq!(
        report.totals,
        ReportCounts {
            events: 9,
            denials: 3,
            approval_required: 1,
            authorizations: 1,
            high_risk_events: 6
        }
    );
    assert_eq!(report.high_risk_threshold, 70);
    assert_eq!(report.total_action_names, 2);
    assert_eq!(report.next_offset, Some(1));
    assert_eq!(report.actions.len(), 1);
    assert_eq!(report.actions[0].action, "a.read");
    assert_eq!(report.actions[0].counts.denials, 3);
    assert_eq!(report.actions[0].counts.high_risk_events, 0);
    let second = reader.report(&q, page(1, 1), risk(70));
    assert_eq!(second.totals, report.totals);
    assert_eq!(second.actions[0].action, "z.write");
    assert_eq!(second.actions[0].counts.events, 6);
    assert_eq!(second.next_offset, None);
    let past = reader.report(&q, page(2, 1), risk(70));
    assert_eq!(past.totals, report.totals);
    assert!(past.actions.is_empty());
    assert_eq!(past.next_offset, None);
    let json = serde_json::to_string(&report).unwrap();
    assert!(!json.contains("secret.action"));
    assert!(!json.contains("resource"));
    assert_eq!(
        reader
            .report(&q, page(0, 10), risk(0))
            .totals
            .high_risk_events,
        9
    );
    assert_eq!(
        reader
            .report(&q, page(0, 10), risk(100))
            .totals
            .high_risk_events,
        0
    );
}
