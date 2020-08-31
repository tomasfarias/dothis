use serde::{self, Deserialize, Serialize};

use super::bool_from_int;
use super::optional_bool_from_int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: u32,
    legacy_id: Option<u32>,
    name: String,
    color: u8,
    parent_id: Option<u32>,
    legacy_parent_id: Option<u32>,
    child_order: i32,
    #[serde(with = "bool_from_int")]
    collapsed: bool,
    shared: bool,
    #[serde(with = "bool_from_int")]
    is_deleted: bool,
    #[serde(with = "bool_from_int")]
    is_archived: bool,
    #[serde(with = "bool_from_int")]
    is_favorite: bool,
    sync_id: Option<u32>,
    inbox_project: Option<bool>,
    team_inbox: Option<bool>,
}
