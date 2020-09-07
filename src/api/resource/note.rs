use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap as Map;

use super::bool_from_int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: u32,
    pub legacy_id: Option<u32>,
    pub posted_uid: u32,
    pub item_id: u32,
    pub legacy_item_id: Option<u32>,
    pub project_id: u32,
    pub legacy_project_id: Option<u32>,
    pub content: String,
    pub file_attachment: FileAttachment,
    pub uids_to_notify: Vec<u32>,
    #[serde(with = "bool_from_int")]
    pub is_deleted: bool,
    pub posted: String,
    pub reactions: Map<String, Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNote {
    pub id: u32,
    pub posted_uid: u32,
    pub project_id: u32,
    pub content: String,
    pub file_attachment: FileAttachment,
    pub uids_to_notify: Vec<u32>,
    #[serde(with = "bool_from_int")]
    pub is_deleted: bool,
    pub posted: String,
    pub reactions: Map<String, Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAttachment {
    pub file_name: String,
    pub file_size: u32,
    pub file_type: String,
    pub file_url: String,
    pub upload_stae: String,
}
