use super::export_record::ExportRecord;
use super::{AuditQuery, AuditReader, ExportPage, PageRequest, ResourceExportPolicy};
use std::io::{self, Write};

impl AuditReader<'_> {
    pub fn export_jsonl<W: Write>(
        &self,
        query: &AuditQuery,
        page: PageRequest,
        policy: ResourceExportPolicy,
        mut writer: W,
    ) -> io::Result<ExportPage> {
        let result = self.timeline(query, page);
        for event in &result.events {
            let record = ExportRecord::new(event, policy);
            serde_json::to_writer(&mut writer, &record).map_err(io::Error::other)?;
            writer.write_all(b"\n")?;
        }
        Ok(ExportPage {
            written: result.events.len(),
            total_matches: result.total_matches,
            next_offset: result.next_offset,
        })
    }
}
