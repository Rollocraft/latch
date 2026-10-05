use super::query_test_fixtures::*;
use crate::*;
use serde_json::Value;

#[test]
fn every_operation_is_tenant_scoped_with_colliding_attribution() {
    let mut events = Vec::new();
    for organization in ["acme", "other", "acme-child"] {
        for timestamp in 0..5 {
            let mut e = event(organization, timestamp, EventResult::Denied);
            e.resource = format!("{organization}-secret");
            events.push(e);
        }
    }
    let reader = AuditReader::new(&events);
    for organization in ["acme", "other", "acme-child", "missing"] {
        for filters in [
            AuditFilters::default(),
            AuditFilters {
                session: Some("shared-session".into()),
                agent: Some("shared-agent".into()),
                owner: Some("owner".into()),
                environment: Some("dev".into()),
                result: Some(EventResult::Denied),
                minimum_risk: Some(risk(70)),
                maximum_risk: Some(risk(70)),
                from_inclusive: Some(0),
                until_exclusive: Some(5),
            },
        ] {
            let q = AuditQuery::new(organization, filters).unwrap();
            let count = if organization == "missing" { 0 } else { 5 };
            for offset in 0..7 {
                let p = reader.timeline(&q, page(offset, 2));
                assert_eq!(p.total_matches, count);
                assert!(p.events.iter().all(|e| e.organization == organization));
                let report = reader.report(&q, page(offset, 2), risk(70));
                assert_eq!(report.organization, organization);
                assert_eq!(report.totals.events, count);
                assert_eq!(report.totals.denials, count);
                assert_eq!(report.totals.high_risk_events, count);
                let mut bytes = Vec::new();
                let exported = reader
                    .export_jsonl(
                        &q,
                        page(offset, 2),
                        ResourceExportPolicy::Include,
                        &mut bytes,
                    )
                    .unwrap();
                assert_eq!(exported.total_matches, count);
                assert_eq!(exported.written, p.events.len());
                assert_eq!(exported.next_offset, p.next_offset);
                for line in String::from_utf8(bytes).unwrap().lines() {
                    let value: Value = serde_json::from_str(line).unwrap();
                    assert_eq!(value["organization"], organization);
                    assert_eq!(value["resource"], format!("{organization}-secret"));
                }
            }
        }
    }
}
