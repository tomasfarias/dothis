use serde::de::{self, Unexpected};
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(a_bool: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(b) = *a_bool {
        return s.serialize_i32(b as i32);
    }
    s.serialize_none()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Some(u8::deserialize(deserializer)?) {
        Some(0) => Ok(Some(false)),
        Some(1) => Ok(Some(true)),
        None => Ok(None),
        Some(other) => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero, one or None",
        )),
    }
}
