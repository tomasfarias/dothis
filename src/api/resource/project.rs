use serde::{self, Deserialize, Serialize};
use serde_json::{self, json};
use std::convert::TryFrom;

use super::bool_int;
use super::color::Color;
use super::{CommandResource, Resource};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub legacy_id: Option<u32>,
    pub name: String,
    pub color: Color,
    pub parent_id: Option<u32>,
    pub legacy_parent_id: Option<u32>,
    pub child_order: i32,
    #[serde(with = "bool_int")]
    pub collapsed: bool,
    pub shared: bool,
    #[serde(with = "bool_int")]
    pub is_deleted: bool,
    #[serde(with = "bool_int")]
    pub is_archived: bool,
    #[serde(with = "bool_int")]
    pub is_favorite: bool,
    pub sync_id: Option<u32>,
    pub inbox_project: Option<bool>,
    pub team_inbox: Option<bool>,
}

impl Resource for Project {
    fn resource(&self) -> String {
        String::from("projects")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddProject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_int::optional")]
    pub is_favorite: Option<bool>,
}

impl Resource for AddProject {
    fn resource(&self) -> String {
        String::from("projects")
    }
}

impl AddProject {
    pub fn new(
        name: &str,
        color: Option<&str>,
        parent_id: Option<u32>,
        child_order: Option<u32>,
        is_favorite: Option<bool>,
    ) -> Self {
        AddProject {
            name: name.to_string(),
            color: match color {
                Some(s) => match Color::try_from(s) {
                    Ok(c) => Some(c),
                    Err(_) => None,
                },
                None => None,
            },
            parent_id: parent_id,
            child_order: child_order,
            is_favorite: is_favorite,
        }
    }
}

impl CommandResource for AddProject {
    fn to_json(&self) -> serde_json::Value {
        json!(self)
    }

    fn command(&self) -> String {
        String::from("project_add")
    }
}
