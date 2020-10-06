use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
pub struct Color {
    code: u8,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.code)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            num => Ok(Color { code: num }),
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match color_to_u8(item) {
            Some(num) => Ok(Color { code: num }),
            None => Err(()),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match u8_to_color(self.code) {
            Some(s) => write!(f, "{}", s),
            None => Err(fmt::Error),
        }
    }
}

fn color_to_u8(color: &str) -> Option<u8> {
    let mut s = String::from(color);
    s.make_ascii_lowercase();

    match s.as_str() {
        // color names as defined by Todoist
        "#b8256f" | "berry red" => Some(30),
        "#db4035" | "red" => Some(31),
        "#ff9933" | "orange" => Some(32),
        "#fad000" | "yellow" => Some(33),
        "#afb83b" | "olive green" => Some(34),
        "#7ecc49" | "lime green" => Some(35),
        "#299438" | "green" => Some(36),
        "#6accbc" | "mint green" => Some(37),
        "#158fad" | "teal" => Some(38),
        "#14aaf5" | "sky blue" => Some(39),
        "#96c3eb" | "light blue" => Some(40),
        "#4073ff" | "blue" => Some(41),
        "#884dff" | "grape" => Some(42),
        "#af38eb" | "violet" => Some(43),
        "#eb96eb" | "lavender" => Some(44),
        "#e05194" | "magenta" => Some(45),
        "#ff8d85" | "salmon" => Some(46),
        "#808080" | "charcoal" => Some(47),
        "#b8b8b8" | "grey" => Some(48),
        "#ccac93" | "tauple" => Some(49),
        other => None,
    }
}

fn u8_to_color(num: u8) -> Option<String> {
    match num {
        // should support hex too?
        30 => Some("Berry Red".to_string()),
        31 => Some("Red".to_string()),
        32 => Some("Orange".to_string()),
        33 => Some("Yellow".to_string()),
        34 => Some("Olive Green".to_string()),
        35 => Some("Lime Green".to_string()),
        36 => Some("Green".to_string()),
        37 => Some("Mint Green".to_string()),
        38 => Some("Teal".to_string()),
        39 => Some("Sky Blue".to_string()),
        40 => Some("Light Blue".to_string()),
        41 => Some("Blue".to_string()),
        42 => Some("Grape".to_string()),
        43 => Some("Violet".to_string()),
        44 => Some("Lavender".to_string()),
        45 => Some("Magenta".to_string()),
        46 => Some("Salmon".to_string()),
        47 => Some("Charcoal".to_string()),
        48 => Some("Grey".to_string()),
        49 => Some("Tauple".to_string()),
        other => None,
    }
}
