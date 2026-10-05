use super::query_test_fixtures::*;
use crate::*;

#[test]
fn risk_and_time_ranges_validate_and_obey_boundaries() {
    for (start, end) in [(10, 10), (11, 10), (u64::MAX, 0)] {
        assert_eq!(
            AuditQuery::new(
                "acme",
                AuditFilters {
                    from_inclusive: Some(start),
                    until_exclusive: Some(end),
                    ..Default::default()
                }
            ),
            Err(QueryError::InvalidTimeWindow)
        );
    }
    assert_eq!(
        AuditQuery::new(
            "acme",
            AuditFilters {
                minimum_risk: Some(risk(100)),
                maximum_risk: Some(risk(0)),
                ..Default::default()
            }
        ),
        Err(QueryError::InvalidRiskRange)
    );
    let events: Vec<_> = [0, 9, 10, 19, 20, u64::MAX]
        .into_iter()
        .map(|t| event("acme", t, EventResult::Succeeded))
        .collect();
    let reader = AuditReader::new(&events);
    for (start, end, expected) in [
        (Some(10), Some(20), vec![10, 19]),
        (None, Some(10), vec![0, 9]),
        (Some(20), None, vec![20, u64::MAX]),
        (Some(u64::MAX), None, vec![u64::MAX]),
        (None, Some(0), vec![]),
        (None, None, vec![0, 9, 10, 19, 20, u64::MAX]),
    ] {
        let q = AuditQuery::new(
            "acme",
            AuditFilters {
                from_inclusive: start,
                until_exclusive: end,
                ..Default::default()
            },
        )
        .unwrap();
        let timestamps: Vec<_> = reader
            .timeline(&q, page(0, 10))
            .events
            .iter()
            .map(|e| e.timestamp)
            .collect();
        assert_eq!(timestamps, expected);
    }
    let events: Vec<_> = [0, 49, 50, 51, 100]
        .into_iter()
        .map(|value| {
            let mut e = event("acme", 0, EventResult::Succeeded);
            e.risk = risk(value);
            e
        })
        .collect();
    let reader = AuditReader::new(&events);
    for (min, max, count) in [
        (None, None, 5),
        (Some(50), Some(50), 1),
        (Some(50), None, 3),
        (None, Some(50), 3),
        (Some(0), Some(100), 5),
    ] {
        let q = AuditQuery::new(
            "acme",
            AuditFilters {
                minimum_risk: min.map(risk),
                maximum_risk: max.map(risk),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(reader.timeline(&q, page(0, 10)).total_matches, count);
    }
}
