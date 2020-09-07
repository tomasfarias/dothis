use serde::{self, Deserialize, Serialize};

use super::bool_from_int;
use super::optional_bool_from_int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: u32,
    pub name: String,
    pub color: u8,
    pub item_order: u32,
    #[serde(with = "bool_from_int")]
    pub is_deleted: bool,
    #[serde(with = "bool_from_int")]
    pub is_favorite: bool,
}
