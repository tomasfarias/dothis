use serde::{self, Deserialize, Serialize};

use super::bool_from_int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub legacy_id: Option<u32>,
    pub name: String,
    pub color: u8,
    pub parent_id: Option<u32>,
    pub legacy_parent_id: Option<u32>,
    pub child_order: i32,
    #[serde(with = "bool_from_int")]
    pub collapsed: bool,
    pub shared: bool,
    #[serde(with = "bool_from_int")]
    pub is_deleted: bool,
    #[serde(with = "bool_from_int")]
    pub is_archived: bool,
    #[serde(with = "bool_from_int")]
    pub is_favorite: bool,
    pub sync_id: Option<u32>,
    pub inbox_project: Option<bool>,
    pub team_inbox: Option<bool>,
}
