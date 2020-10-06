use serde::{self, Deserialize, Serialize};

use super::bool_int;
use super::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    id: u32,
    name: String,
    query: String,
    color: Color,
    item_order: i32,
    #[serde(with = "bool_int")]
    is_deleted: bool,
    #[serde(with = "bool_int")]
    is_favorite: bool,
}
