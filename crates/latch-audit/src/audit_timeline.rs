use super::{AuditPage, AuditQuery, PageRequest};
use super::{page_request::next_offset, timeline_order::timeline_order};
use crate::AuditEvent;

pub struct AuditReader<'a> {
    pub(super) events: &'a [AuditEvent],
}

impl<'a> AuditReader<'a> {
    pub fn new(events: &'a [AuditEvent]) -> Self {
        Self { events }
    }

    pub fn timeline(&self, query: &AuditQuery, page: PageRequest) -> AuditPage<'a> {
        let mut events: Vec<_> = self
            .events
            .iter()
            .filter(|event| query.matches(event))
            .collect();
        events.sort_unstable_by(|left, right| timeline_order(left, right));
        let total_matches = events.len();
        AuditPage {
            events: events
                .into_iter()
                .skip(page.offset())
                .take(page.limit())
                .collect(),
            total_matches,
            next_offset: next_offset(page, total_matches),
        }
    }
}
