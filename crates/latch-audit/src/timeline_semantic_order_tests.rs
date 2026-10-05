use super::query_test_fixtures::*;
use crate::*;

#[test]
fn timeline_has_semantic_tie_order_independent_of_input_order() {
    let results = [
        EventResult::ApprovalRequired,
        EventResult::Denied,
        EventResult::ReplayRejected,
        EventResult::BudgetRejected,
        EventResult::Authorized,
        EventResult::PreparationFailed,
        EventResult::Started,
        EventResult::Succeeded,
        EventResult::Failed,
    ];
    let mut events: Vec<_> = results
        .into_iter()
        .map(|result| event("acme", 10, result))
        .collect();
    let expected = events.clone();
    let q = query("acme");
    for _ in 0..events.len() {
        events.rotate_left(1);
        let actual: Vec<_> = AuditReader::new(&events)
            .timeline(&q, page(0, 100))
            .events
            .into_iter()
            .cloned()
            .collect();
        assert_eq!(actual, expected);
    }
    let names: Vec<_> = export(&AuditReader::new(&events), &q, ResourceExportPolicy::Omit)
        .into_iter()
        .map(|v| v["result"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        names,
        [
            "approval_required",
            "denied",
            "replay_rejected",
            "budget_rejected",
            "authorized",
            "preparation_failed",
            "started",
            "succeeded",
            "failed"
        ]
    );
    let mut early = event("acme", 9, EventResult::Failed);
    early.action_id = "z".into();
    events.push(early.clone());
    assert_eq!(
        AuditReader::new(&events).timeline(&q, page(0, 1)).events,
        vec![&early]
    );
}
