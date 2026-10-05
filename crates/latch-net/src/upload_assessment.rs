use super::{ExfiltrationMonitor, Response};
use crate::Connection;

impl ExfiltrationMonitor {
    /// Judge one connection. `familiar` says whether the destination is one the
    /// policy names outright; a destination reached through a wildcard is not
    /// familiar, because that is exactly where an unknown collector would sit.
    pub fn assess(&self, connection: &Connection, familiar: bool, now: u64) -> Response {
        let mut response = Response::Allow;
        if !familiar && !self.recent_reads(now).is_empty() {
            response = response.max(Response::RequireApproval);
        }
        let sent = connection.bytes_sent;
        if sent >= self.freeze_bytes {
            response = response.max(Response::Freeze);
        } else if sent >= self.approval_bytes {
            response = response.max(Response::RequireApproval);
        } else if sent >= self.warn_bytes {
            response = response.max(Response::Warn);
        }
        response
    }
}
