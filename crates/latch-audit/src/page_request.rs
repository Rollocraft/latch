use super::{MAX_PAGE_SIZE, QueryError};
use crate::AuditEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    offset: usize,
    limit: usize,
}

impl PageRequest {
    pub fn new(offset: usize, limit: usize) -> Result<Self, QueryError> {
        if limit == 0 || limit > MAX_PAGE_SIZE || offset.checked_add(limit).is_none() {
            return Err(QueryError::InvalidPage);
        }
        Ok(Self { offset, limit })
    }

    pub fn offset(self) -> usize {
        self.offset
    }

    pub fn limit(self) -> usize {
        self.limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditPage<'a> {
    pub events: Vec<&'a AuditEvent>,
    pub total_matches: usize,
    pub next_offset: Option<usize>,
}

pub(super) fn next_offset(page: PageRequest, total: usize) -> Option<usize> {
    let end = page.offset() + page.limit();
    (end < total).then_some(end)
}
