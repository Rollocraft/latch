use crate::terminal_text::{CLIPPED, terminal_text};
use crate::{MAX_OUTPUT_BYTES, MAX_ROWS, Rendered};

/// Byte-bounded line accumulator shared by every renderer.
#[derive(Default)]
pub(crate) struct Output {
    pub(crate) text: String,
    pub(crate) truncated: bool,
    full: bool,
}

impl Output {
    pub(crate) fn line(&mut self, line: &str) {
        if self.full {
            return;
        }
        if self.text.len() + line.len() + 1 > MAX_OUTPUT_BYTES - CLIPPED.len() - 1 {
            self.text.push_str(CLIPPED);
            self.text.push('\n');
            self.truncated = true;
            self.full = true;
        } else {
            self.text.push_str(line);
            self.text.push('\n');
        }
    }

    pub(crate) fn field(&mut self, label: &str, value: &str) {
        let escaped = terminal_text(value);
        self.truncated |= escaped.truncated;
        self.line(&format!("{label}: {}", escaped.text));
    }

    pub(crate) fn omitted(&mut self, total: usize) {
        if total > MAX_ROWS {
            self.truncated = true;
            self.line(&format!(
                "... {} additional entries omitted",
                total - MAX_ROWS
            ));
        }
    }

    pub(crate) fn finish(self) -> Rendered {
        Rendered {
            text: self.text,
            truncated: self.truncated,
        }
    }
}
