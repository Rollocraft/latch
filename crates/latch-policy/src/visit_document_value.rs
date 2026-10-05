use super::bounded_document_value::{BoundedValue, consume_text};
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::fmt;

impl<'de> Visitor<'de> for BoundedValue<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded JSON-compatible policy data")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| de::Error::custom("non-finite numbers are not supported"))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Value, E> {
        consume_text(self.bytes, value)?;
        Ok(Value::String(value.into()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Value, E> {
        consume_text(self.bytes, &value)?;
        Ok(Value::String(value))
    }

    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_none<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(BoundedValue {
            depth: self.depth + 1,
            remaining: self.remaining,
            bytes: self.bytes,
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut mapping: A) -> Result<Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = mapping.next_key::<String>()? {
            consume_text(self.bytes, &key)?;
            if values.contains_key(&key) {
                return Err(de::Error::custom("duplicate mapping key"));
            }
            let value = mapping.next_value_seed(BoundedValue {
                depth: self.depth + 1,
                remaining: self.remaining,
                bytes: self.bytes,
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}
