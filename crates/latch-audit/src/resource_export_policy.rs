#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResourceExportPolicy {
    #[default]
    Omit,
    Redact,
    Include,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportPage {
    pub written: usize,
    pub total_matches: usize,
    pub next_offset: Option<usize>,
}
