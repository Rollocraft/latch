use super::query_test_fixtures::*;
use crate::*;

#[test]
fn optional_filters_are_exact_conjunctive_and_shared_by_reports_and_exports() {
    let wanted = event("acme", 10, EventResult::Succeeded);
    let mut events = vec![wanted.clone()];
    for field in 0..9 {
        let mut other = wanted.clone();
        match field {
            0 => other.organization = "other".into(),
            1 => other.session = "other".into(),
            2 => other.agent = "other".into(),
            3 => other.owner = "other".into(),
            4 => other.environment = "other".into(),
            5 => other.result = EventResult::Denied,
            6 => other.risk = risk(69),
            7 => other.risk = risk(71),
            _ => other.timestamp = 11,
        }
        events.push(other);
    }
    let filters = AuditFilters {
        session: Some(wanted.session.clone()),
        agent: Some(wanted.agent.clone()),
        owner: Some(wanted.owner.clone()),
        environment: Some(wanted.environment.clone()),
        result: Some(wanted.result),
        minimum_risk: Some(risk(70)),
        maximum_risk: Some(risk(70)),
        from_inclusive: Some(10),
        until_exclusive: Some(11),
    };
    let q = AuditQuery::new("acme", filters.clone()).unwrap();
    assert_eq!(q.filters(), &filters);
    let reader = AuditReader::new(&events);
    assert_eq!(reader.timeline(&q, page(0, 10)).events, vec![&wanted]);
    assert_eq!(reader.report(&q, page(0, 10), risk(70)).totals.events, 1);
    assert_eq!(export(&reader, &q, ResourceExportPolicy::Omit).len(), 1);
    for filters in [
        AuditFilters {
            session: Some("shared".into()),
            ..Default::default()
        },
        AuditFilters {
            agent: Some("*".into()),
            ..Default::default()
        },
        AuditFilters {
            owner: Some("OWNER".into()),
            ..Default::default()
        },
        AuditFilters {
            environment: Some("dev ".into()),
            ..Default::default()
        },
    ] {
        let q = AuditQuery::new("acme", filters).unwrap();
        assert_eq!(reader.timeline(&q, page(0, 10)).total_matches, 0);
    }
}
