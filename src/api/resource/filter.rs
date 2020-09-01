use serde::{self, Deserialize, Serialize};

use super::bool_from_int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    id: u32,
    name: String,
    query: String,
    color: u8,
    item_order: i32,
    #[serde(with = "bool_from_int")]
    is_deleted: bool,
    #[serde(with = "bool_from_int")]
    is_favorite: bool,
}
