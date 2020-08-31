use serde::de::{self, Unexpected};
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(a_bool: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let b = *a_bool {
        return s.serialize_i32(b as i32);
    }
    s.serialize_none()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
