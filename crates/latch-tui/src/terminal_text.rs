use crate::MAX_FIELD_BYTES;

pub(crate) const CLIPPED: &str = "... [clipped]";

/// Terminal-safe rendering of one field: debug-escaped, byte-bounded output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
    pub text: String,
    pub truncated: bool,
}

/// Escapes a single field value for terminal output, clipping at the field limit.
pub fn terminal_text(value: &str) -> Rendered {
    let mut text = String::new();
    for character in value.chars() {
        let escaped: String = character.escape_debug().collect();
        if text.len() + escaped.len() > MAX_FIELD_BYTES - CLIPPED.len() {
            text.push_str(CLIPPED);
            return Rendered {
                text,
                truncated: true,
            };
        }
        text.push_str(&escaped);
    }
    Rendered {
        text,
        truncated: false,
    }
}
