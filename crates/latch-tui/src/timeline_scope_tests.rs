use crate::presentation_test_fixtures::*;
use crate::*;
use latch_audit::{
    AuditEvent, AuditFilters, AuditPage, AuditQuery, AuditReader, EventResult, PageRequest,
};

#[test]
fn timeline_consumes_query_pages_and_hides_foreign_events() {
    let session = session();
    let late = AuditEvent::for_action(&session, &action(), EventResult::Succeeded, 30);
    let mut early = AuditEvent::for_action(&session, &action(), EventResult::ApprovalRequired, 20);
    early.resource = "repo/\x1b[2J\nforged".into();
    let events = [late, early];
    let query = AuditQuery::new(
        "acme",
        AuditFilters {
            session: Some(session.id().into()),
            ..Default::default()
        },
    )
    .unwrap();
    let page = AuditReader::new(&events).timeline(&query, PageRequest::new(0, 1).unwrap());
    let rendered = render_session_timeline(&session, &page);
    assert!(rendered.text.contains("Time: 20"));
    assert!(!rendered.text.contains("Time: 30"));
    assert!(rendered.text.contains("next offset: 1"));
    assert_safe(&rendered.text);
    let mut foreign = events[0].clone();
    foreign.organization = "other".into();
    foreign.resource = "FOREIGN_SECRET".into();
    let page = AuditPage {
        events: vec![&foreign],
        total_matches: 1,
        next_offset: None,
    };
    let rendered = render_session_timeline(&session, &page);
    assert!(rendered.text.contains("SCOPE MISMATCH"));
    assert!(!rendered.text.contains("FOREIGN_SECRET"));
}
