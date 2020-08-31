use serde::{self, Deserialize, Serialize};

use super::bool_from_int;
use super::optional_bool_from_int;

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
    #[serde(with = "bool_from_int")]
    collapsed: bool,
    labels: Vec<u32>,
    added_by_uid: Option<u32>,
    assigned_by_uid: Option<i32>,
    responsible_uid: Option<i32>,
    #[serde(with = "bool_from_int")]
    checked: bool,
    #[serde(with = "bool_from_int")]
    in_history: bool,
    #[serde(with = "bool_from_int")]
    is_deleted: bool,
    sync_id: Option<u32>,
    date_completed: Option<String>,
    date_added: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    date: String,
    timezone: Option<String>,
    string: String,
    lang: String,
    is_recurring: bool,
}
