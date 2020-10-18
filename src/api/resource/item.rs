use serde::{self, Deserialize, Serialize};
use serde_json::{self, json};

use super::bool_int;
use super::{CommandResource, Resource};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub legacy_id: Option<u32>,
    pub user_id: u32,
    pub project_id: u32,
    pub legay_project_id: Option<u32>,
    pub content: String,
    pub due: Option<DueDate>,
    pub priority: i32,
    pub parent_id: Option<u32>,
    pub legacy_parent_id: Option<u32>,
    pub child_order: i32,
    pub section_id: Option<u32>,
    pub day_order: i32,
    #[serde(with = "bool_int")]
    pub collapsed: bool,
    pub labels: Vec<u32>,
    pub added_by_uid: Option<u32>,
    pub assigned_by_uid: Option<i32>,
    pub responsible_uid: Option<i32>,
    #[serde(with = "bool_int")]
    pub checked: bool,
    #[serde(with = "bool_int")]
    pub in_history: bool,
    #[serde(with = "bool_int")]
    pub is_deleted: bool,
    pub sync_id: Option<u32>,
    pub date_completed: Option<String>,
    pub date_added: String,
}

impl Resource for Item {
    fn resource(&self) -> String {
        String::from("items")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<u32>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<DueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_int::optional")]
    pub collapsed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_by_uid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsible_uid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reminder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_parse_labels: Option<bool>,
}

impl Resource for AddItem {
    fn resource(&self) -> String {
        String::from("items")
    }
}

impl CommandResource for AddItem {
    fn to_json(&self) -> serde_json::Value {
        json!(self)
    }

    fn command(&self) -> String {
        String::from("item_add")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reminder {
    id: u32,
    notify_uid: u32,
    item_uid: u32,
    service: String,
    #[serde(rename = "type")]
    type_: String,
    due: DueDate,
    mm_offset: Option<i32>,
    name: String,
    loc_lat: String,
    loc_long: String,
    loc_trigger: String,
    radius: i32,
    #[serde(with = "bool_int")]
    is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDate {
    pub date: String,
    pub timezone: Option<String>,
    pub string: String,
    pub lang: String,
    pub is_recurring: bool,
}
