use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: u32,
    legacy_id: Option<u32>,
    posted_uid: u32,
    item_id: u32,
    legacy_item_id: Option<u32>,
    project_id: u32,
    legacy_project_id: Option<u32>,
    content: String,
    file_attachment: FileAttachment,
    uids_to_notify: Vec<u32>,
    is_deleted: bool,
    posted: String,
    reactions: Map<String, Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAttachment {
    file_name: String,
    file_size: u32,
    file_type: String,
    file_url: String,
    upload_stae: String,
}
