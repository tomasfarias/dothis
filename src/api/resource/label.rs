use serde::{self, Deserialize, Serialize};

use super::bool_int;
use super::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: u32,
    pub name: String,
    pub color: Color,
    pub item_order: u32,
    #[serde(with = "bool_int")]
    pub is_deleted: bool,
    #[serde(with = "bool_int")]
    pub is_favorite: bool,
}
