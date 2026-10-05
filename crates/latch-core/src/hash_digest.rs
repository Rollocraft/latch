#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest([u8; 32]);

impl Digest {
    pub const LENGTH: usize = 32;
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        self.0.iter().fold(String::with_capacity(64), |mut out, b| {
            out.push(char::from_digit((b >> 4) as u32, 16).expect("nibble"));
            out.push(char::from_digit((b & 0xf) as u32, 16).expect("nibble"));
            out
        })
    }

    /// Lowercase hexadecimal only, so a digest has exactly one encoding.
    pub fn parse_hex(text: &str) -> Option<Self> {
        if text.len() != 64 || text.bytes().any(|b| b.is_ascii_uppercase()) {
            return None;
        }
        let mut bytes = [0u8; 32];
        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = u8::from_str_radix(text.get(index * 2..index * 2 + 2)?, 16).ok()?;
        }
        Some(Self(bytes))
    }
}
