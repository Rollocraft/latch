use super::query_test_fixtures::*;
use crate::*;

#[test]
fn timeline_ties_are_total_and_duplicates_are_retained() {
    let base = event("acme", 10, EventResult::Succeeded);
    let mut events = vec![base.clone(), base.clone()];
    for field in 0..9 {
        let mut e = base.clone();
        match field {
            0 => e.session = "z".into(),
            1 => e.agent = "z".into(),
            2 => e.action_id = "z".into(),
            3 => e.owner = "z".into(),
            4 => e.environment = "z".into(),
            5 => e.action = "z".into(),
            6 => e.risk = risk(100),
            7 => e.resource = "z".into(),
            _ => e.policies = vec!["z".into()],
        }
        events.push(e);
    }
    let q = query("acme");
    let expected: Vec<_> = AuditReader::new(&events)
        .timeline(&q, page(0, 100))
        .events
        .into_iter()
        .cloned()
        .collect();
    for _ in 0..events.len() {
        events.rotate_left(1);
        events.reverse();
        let actual: Vec<_> = AuditReader::new(&events)
            .timeline(&q, page(0, 100))
            .events
            .into_iter()
            .cloned()
            .collect();
        assert_eq!(actual, expected);
    }
    assert_eq!(expected.iter().filter(|e| **e == base).count(), 2);
}
