mod bool_int;
mod color;
mod filter;
mod item;
mod label;
mod note;
mod project;

pub use self::filter::Filter;
pub use self::item::{Item, Reminder};
pub use self::label::Label;
pub use self::note::{Note, ProjectNote};
pub use self::project::{NewProject, Project};

pub enum Resource {
    Project,
    Item,
    Reminder,
    Label,
    Note,
    Filter,
    ProjectNote,
}

pub enum NewResource {
    NewProject,
}

pub trait ToJson {
    fn to_json(&self) -> serde_json::Value;
}
