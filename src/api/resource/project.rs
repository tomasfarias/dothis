use serde::{self, Deserialize, Serialize};
use serde_json::{self, json};
use std::convert::TryFrom;

use super::bool_int;
use super::color::Color;
use super::ToJson;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub color: Option<Color>,
    pub parent_id: Option<u32>,
    pub child_order: Option<u32>,
    #[serde(with = "bool_int::optional")]
    pub is_favorite: Option<bool>,
}

impl NewProject {
    pub fn new(
        name: &str,
        color: Option<&str>,
        child_order: Option<u32>,
        is_favorite: Option<bool>,
    ) -> Self {
        NewProject {
            name: name.to_string(),
            color: match color {
                Some(s) => match Color::try_from(s) {
                    Ok(c) => Some(c),
                    Err(_) => None,
                },
                None => None,
            },
            parent_id: None,
            child_order: child_order,
            is_favorite: is_favorite,
        }
    }
}

impl ToJson for NewProject {
    fn to_json(&self) -> serde_json::Value {
        json!(self)
    }
}
