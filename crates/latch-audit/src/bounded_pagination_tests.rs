use super::query_test_fixtures::*;
use crate::*;

#[test]
fn pagination_is_bounded_overflow_safe_and_complete() {
    for (offset, limit) in [
        (0, 0),
        (0, MAX_PAGE_SIZE + 1),
        (usize::MAX, 1),
        (usize::MAX - 1, 2),
    ] {
        assert_eq!(
            PageRequest::new(offset, limit),
            Err(QueryError::InvalidPage)
        );
    }
    assert_eq!(page(usize::MAX - 1, 1).offset(), usize::MAX - 1);
    assert_eq!(page(0, MAX_PAGE_SIZE).limit(), MAX_PAGE_SIZE);
    let events: Vec<_> = (0..(MAX_PAGE_SIZE as u64 + 3))
        .rev()
        .map(|t| event("acme", t, EventResult::Succeeded))
        .collect();
    let reader = AuditReader::new(&events);
    let q = query("acme");
    let mut offset = 0;
    let mut timestamps = Vec::new();
    loop {
        let p = reader.timeline(&q, page(offset, MAX_PAGE_SIZE));
        assert_eq!(p.total_matches, events.len());
        assert!(p.events.len() <= MAX_PAGE_SIZE);
        timestamps.extend(p.events.iter().map(|e| e.timestamp));
        match p.next_offset {
            Some(next) => offset = next,
            None => break,
        }
    }
    assert_eq!(timestamps, (0..events.len() as u64).collect::<Vec<_>>());
    for offset in [events.len(), events.len() + 1, usize::MAX - 1] {
        let p = reader.timeline(&q, page(offset, 1));
        assert!(p.events.is_empty());
        assert_eq!(p.next_offset, None);
        assert_eq!(p.total_matches, events.len());
    }
    assert_eq!(reader.timeline(&q, page(0, 1)).next_offset, Some(1));
    assert_eq!(
        reader.timeline(&q, page(events.len() - 1, 1)).next_offset,
        None
    );
}
