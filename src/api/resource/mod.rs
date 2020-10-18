mod bool_int;
mod color;
mod filter;
mod item;
mod label;
mod note;
mod project;

pub use self::filter::Filter;
pub use self::item::{AddItem, DueDate, Item, Reminder};
pub use self::label::Label;
pub use self::note::{Note, ProjectNote};
pub use self::project::{AddProject, Project};

pub trait Resource {
    fn resource(&self) -> String;
}

pub trait CommandResource {
    fn to_json(&self) -> serde_json::Value;
    fn command(&self) -> String;
}

pub enum AddResource {
    Item(AddItem),
    Project(AddProject),
}

impl Resource for AddResource {
    fn resource(&self) -> String {
        match self {
            AddResource::Item(res) => res.resource(),
            AddResource::Project(res) => res.resource(),
        }
    }
}

impl CommandResource for AddResource {
    fn command(&self) -> String {
        match self {
            AddResource::Item(add) => add.command(),
            AddResource::Project(add) => add.command(),
        }
    }

    fn to_json(&self) -> serde_json::Value {
        match self {
            AddResource::Item(add) => add.to_json(),
            AddResource::Project(add) => add.to_json(),
        }
    }
}
