use crate::MAX_POLICY_DOCUMENT_DEPTH;
use serde::Deserializer;
use serde::de::{self, DeserializeSeed};
use serde_json::Value;

pub(super) struct BoundedValue<'a> {
    pub(super) depth: usize,
    pub(super) remaining: &'a mut usize,
    pub(super) bytes: &'a mut usize,
}

pub(super) fn consume_text<E: de::Error>(remaining: &mut usize, value: &str) -> Result<(), E> {
    *remaining = remaining
        .checked_sub(value.len())
        .ok_or_else(|| de::Error::custom("policy document expanded text limit exceeded"))?;
    Ok(())
}

impl<'de> DeserializeSeed<'de> for BoundedValue<'_> {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        if self.depth > MAX_POLICY_DOCUMENT_DEPTH || *self.remaining == 0 {
            return Err(de::Error::custom(
                "policy document complexity limit exceeded",
            ));
        }
        *self.remaining -= 1;
        deserializer.deserialize_any(self)
    }
}
