use serde::{self, Deserialize, Serialize};

use super::bool_from_int;
use super::optional_bool_from_int;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    id: u32,
    name: String,
    color: u8,
    item_order: u32,
    #[serde(with = "bool_from_int")]
    is_deleted: bool,
    #[serde(with = "bool_from_int")]
    is_favorite: bool,
}
