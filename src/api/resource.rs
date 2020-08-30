use serde::de::{self, Deserializer, Unexpected};
use serde::{self, Deserialize, Serialize, Serializer};
use serde_json::{self, Result};

pub trait Resource {
    fn to_command(&self) -> Result<String>;
    fn name() -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: u32,
    legacy_id: Option<u32>,
    name: String,
    color: Option<u32>,
    parent_id: Option<u32>,
    legacy_parent_id: Option<u32>,
    child_order: Option<i32>,
    #[serde(with = "optional_bool_from_int")]
    collapsed: Option<bool>,
    shared: Option<bool>,
    #[serde(with = "optional_bool_from_int")]
    is_deleted: Option<bool>,
    #[serde(with = "optional_bool_from_int")]
    is_archived: Option<bool>,
    #[serde(with = "optional_bool_from_int")]
    is_favorite: Option<bool>,
    sync_id: Option<u32>,
    inbox_project: Option<bool>,
    team_inbox: Option<bool>,
}

impl Resource for Project {
    fn to_command(&self) -> Result<String> {
        serde_json::to_string(&self)
    }

    fn name() -> String {
        "projects".to_owned()
    }
}

mod optional_bool_from_int {
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
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    legacy_id: Option<u32>,
    user_id: u32,
    project_id: u32,
    legay_project_id: Option<u32>,
    content: String,
    due: DueDate,
    priority: i32,
    parent_id: Option<u32>,
    legacy_parent_id: Option<u32>,
    child_order: i32,
    section_id: Option<u32>,
    day_order: i32,
    #[serde(with = "optional_bool_from_int")]
    collapsed: Option<bool>,
    labels: Vec<u32>,
    added_by_uid: Option<u32>,
    assigned_by_uid: Option<i32>,
    responsible_uid: Option<i32>,
    #[serde(with = "optional_bool_from_int")]
    checked: Option<bool>,
    #[serde(with = "optional_bool_from_int")]
    in_history: Option<bool>,
    #[serde(with = "optional_bool_from_int")]
    is_deleted: Option<bool>,
    sync_id: Option<u32>,
    date_completed: Option<String>,
    date_added: String,
}

impl Resource for Item {
    fn to_command(&self) -> Result<String> {
        serde_json::to_string(&self)
    }

    fn name() -> String {
        "items".to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    date: String,
    timezone: Option<String>,
    string: String,
    lang: String,
    is_recurring: bool,
}
