use super::query_test_fixtures::*;
use crate::*;

#[test]
fn empty_queries_produce_empty_pages_reports_and_exports() {
    let events = [event("other", 0, EventResult::Denied)];
    for input in [&events[..], &[][..]] {
        let reader = AuditReader::new(input);
        let q = query("acme");
        let p = reader.timeline(&q, page(0, 1));
        assert!(p.events.is_empty());
        assert_eq!(p.total_matches, 0);
        assert_eq!(p.next_offset, None);
        let report = reader.report(&q, page(0, 1), risk(70));
        assert_eq!(report.totals, ReportCounts::default());
        assert_eq!(report.total_action_names, 0);
        assert!(report.actions.is_empty());
        assert_eq!(report.next_offset, None);
        let mut bytes = Vec::new();
        assert_eq!(
            reader
                .export_jsonl(&q, page(0, 1), ResourceExportPolicy::Omit, &mut bytes)
                .unwrap(),
            ExportPage {
                written: 0,
                total_matches: 0,
                next_offset: None
            }
        );
        assert!(bytes.is_empty());
    }
}
