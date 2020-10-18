use std::collections::HashMap;

use clap::ArgMatches;
use prettytable::{format, Table};

use dothis::api::client::{TodoistClient, TodoistCommand, TodoistResponse};
use dothis::api::resource::{
    AddItem, AddProject, AddResource, CommandResource, DueDate, Filter, Item, Label, Note, Project,
    ProjectNote, Reminder, Resource,
};

use super::error::DothisError;

pub trait Command {
    // list command should eventually support other outputs besides stdout via an argument
    fn execute(&mut self) -> Result<(), DothisError>;
}

pub enum Commands {
    ListCommand,
    AddCommand,
}

pub struct ListCommand {
    client: TodoistClient,
    resource_type: String,
    table: Table,
}

impl ListCommand {
    pub fn new(token: &str, resource_type: &str, table: Table) -> ListCommand {
        ListCommand {
            client: TodoistClient::new(token),
            resource_type: resource_type.to_string(),
            table: table,
        }
    }

    fn resource_types(&self) -> Result<Vec<&str>, DothisError> {
        match self.resource_type.as_str() {
            "tasks" => Ok(vec!["items", "projects"]),
            "projects" => Ok(vec!["projects"]),
            "labels" => Ok(vec!["labels"]),
            "notes" => Ok(vec!["notes", "projects", "items"]),
            _ => return Err(DothisError::UnknownResource),
        }
    }

    fn get_response(&mut self) -> Result<TodoistResponse, DothisError> {
        let resources = self.resource_types()?;

        match self.client.get_resources(resources) {
            Ok(response) => Ok(response),
            Err(err) => return Err(DothisError::ApiError(err)),
        }
    }

    fn set_table_title(&mut self) -> Result<(), DothisError> {
        match self.resource_type.as_str() {
            "tasks" => self
                .table
                .set_titles(row!["Project", "Added", "Due", "Content"]),
            "projects" => self.table.set_titles(row!["Project", "Parent"]),
            "notes" => self.table.set_titles(row!["Name", "Favorite", "Deleted"]),
            "labels" => self.table.set_titles(row![""]),
            other => return Err(DothisError::UnknownResource),
        };

        Ok(())
    }

    fn set_table_rows(&mut self, response: TodoistResponse) -> Result<(), DothisError> {
        match self.resource_type.as_str() {
            "tasks" => {
                let projects: Vec<Project> = match response.projects {
                    Some(projects) => projects,
                    None => return Err(DothisError::EmptyResponseError),
                };
                let tasks: Vec<Item> = match response.items {
                    Some(tasks) => tasks,
                    None => return Err(DothisError::EmptyResponseError),
                };

                for project in projects.iter() {
                    for task in tasks.iter().filter(|t| t.project_id == project.id) {
                        self.table.add_row(row![
                            project.name,
                            task.date_added,
                            task.due.as_ref().map_or("", |d| &d.string),
                            task.content
                        ]);
                    }
                }
            }
            "projects" => {
                let projects: Vec<Project> = match response.projects {
                    Some(projects) => projects,
                    None => return Err(DothisError::EmptyResponseError),
                };

                let mut projects_map: HashMap<u32, Project> = HashMap::new();
                projects_map.extend(projects.into_iter().map(|p| (p.id, p)));

                for (_, project) in projects_map.iter() {
                    match project.parent_id {
                        Some(parent_id) => self.table.add_row(row![
                            project.name,
                            projects_map.get(&parent_id).map_or("", |p| &p.name)
                        ]),
                        None => self.table.add_row(row![project.name, ""]),
                    };
                }
            }
            "notes" => {
                let notes = match response.notes {
                    Some(notes) => notes,
                    None => return Err(DothisError::EmptyResponseError),
                };

                let tasks: Vec<Item> = match response.items {
                    Some(tasks) => tasks,
                    None => return Err(DothisError::EmptyResponseError),
                };

                let projects: Vec<Project> = match response.projects {
                    Some(projects) => projects,
                    None => return Err(DothisError::EmptyResponseError),
                };

                // There has to be a better way to do this
                for note in notes.iter() {
                    for task in tasks.iter() {
                        for project in projects.iter() {
                            if note.project_id == project.id && note.item_id == task.id {
                                self.table
                                    .add_row(row![project.name, task.id, note.content]);
                            }
                        }
                    }
                }
            }
            "labels" => {
                let labels = match response.labels {
                    Some(labels) => labels,
                    None => return Err(DothisError::EmptyResponseError),
                };

                for label in labels.iter() {
                    self.table
                        .add_row(row![label.name, label.is_favorite, label.is_deleted]);
                }
            }
            other => return Err(DothisError::UnknownResource),
        };

        Ok(())
    }

    fn format_table(&mut self) {
        let tbl_format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Title],
                format::LineSeparator::new('=', ' ', ' ', ' '),
            )
            .build();
        self.table.set_format(tbl_format);
    }

    fn build_table(&mut self, response: TodoistResponse) -> Result<(), DothisError> {
        self.set_table_title()?;
        self.set_table_rows(response)?;
        self.format_table();

        self.table.printstd();
        Ok(())
    }
}

impl Command for ListCommand {
    fn execute(&mut self) -> Result<(), DothisError> {
        let response = self.get_response()?;
        self.build_table(response)
    }
}

pub struct AddCommand<'a> {
    client: TodoistClient,
    resource_type: String,
    args: ArgMatches<'a>,
}

impl<'a> AddCommand<'a> {
    pub fn new(token: &str, resource_type: &str, args: ArgMatches<'a>) -> AddCommand<'a> {
        AddCommand {
            client: TodoistClient::new(token),
            resource_type: resource_type.to_string(),
            args: args,
        }
    }

    pub fn get_new_resource(&self) -> Result<AddResource, DothisError> {
        match self.resource_type.as_str() {
            "tasks" => Ok(AddResource::Item(AddItem {
                content: self
                    .args
                    .value_of("content")
                    .expect("task content is required")
                    .to_string(),
                project_id: self
                    .args
                    .value_of("project_id")
                    .map_or(None, |v| v.parse::<u32>().ok()),
                due: None,
                priority: None,
                parent_id: None,
                child_order: None,
                section_id: None,
                day_order: None,
                collapsed: None,
                labels: None,
                assigned_by_uid: None,
                responsible_uid: None,
                auto_reminder: None,
                auto_parse_labels: None,
            })),
            "projects" => Ok(AddResource::Project(AddProject::new(
                self.args
                    .value_of("name")
                    .expect("project name is required"),
                self.args.value_of("color"),
                self.args
                    .value_of("parent_id")
                    .map_or(None, |v| v.parse::<u32>().ok()),
                self.args
                    .value_of("child_order")
                    .map_or(None, |v| v.parse::<u32>().ok()),
                self.args
                    .value_of("is_favorite")
                    .map_or(None, |v| v.parse::<bool>().ok()),
            ))),
            _ => return Err(DothisError::UnknownResource),
        }
    }
}

impl<'a> Command for AddCommand<'a> {
    fn execute(&mut self) -> Result<(), DothisError> {
        let new_resource = self.get_new_resource()?;
        match self.client.sync_resources(vec![new_resource]) {
            Ok(response) => Ok(()),
            Err(e) => Err(DothisError::ApiError(e)),
        }
    }
}
