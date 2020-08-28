use serde::{ self, Deserialize, Serialize };
use serde_json::{ self, Result };

pub trait Resource {
    fn to_command(&self) -> Result<String>;
    fn name() -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: i32,
    name: String,
    color: Option<i32>,
    parent_id: Option<i32>,
    child_order: Option<i32>,
    collapsed: Option<bool>,
    shared: Option<bool>,
    is_deleted: Option<bool>,
    is_archived: Option<bool>,
    is_favorite: Option<bool>,
    sync_id: Option<i32>,
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
