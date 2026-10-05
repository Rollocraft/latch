use super::{ExfiltrationMonitor, SensitiveRead};

impl ExfiltrationMonitor {
    pub fn is_sensitive(&self, resource: &str) -> bool {
        let resource = resource.to_ascii_lowercase();
        self.markers.iter().any(|marker| resource.contains(marker))
    }

    /// Record a read. Returns whether it was treated as sensitive, so a caller
    /// can show the operator why a later connection was questioned.
    pub fn observe_read(&mut self, resource: &str, bytes: u64, at: u64) -> bool {
        if !self.is_sensitive(resource) {
            return false;
        }
        self.reads.push(SensitiveRead {
            resource: resource.to_owned(),
            bytes,
            at,
        });
        true
    }

    /// Sensitive reads still inside the correlation window at `now`.
    pub fn recent_reads(&self, now: u64) -> Vec<&SensitiveRead> {
        self.reads
            .iter()
            .filter(|read| read.at <= now && now - read.at <= self.window)
            .collect()
    }

    /// Drop reads older than the window, so a long session does not grow
    /// without bound. Anything dropped can no longer raise a correlation.
    pub fn forget_before(&mut self, at: u64) {
        self.reads.retain(|read| read.at >= at);
    }
}
