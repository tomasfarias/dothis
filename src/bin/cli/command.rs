use std::collections::HashMap;

use prettytable::{format, Table};

use dothis::api::client::{TodoistClient, TodoistCommand, TodoistResponse};
use dothis::api::resource::{
    Filter, Item, Label, NewProject, Note, Project, ProjectNote, Reminder, Resource, ToJson,
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

    fn get_resource_types(&self) -> Result<Vec<String>, DothisError> {
        match self.resource_type.as_str() {
            "tasks" => Ok(vec!["items".to_string(), "projects".to_string()]),
            "projects" => Ok(vec!["projects".to_string()]),
            "labels" => Ok(vec!["labels".to_owned()]),
            "notes" => Ok(vec![
                "notes".to_owned(),
                "projects".to_owned(),
                "items".to_owned(),
            ]),
            _ => return Err(DothisError::UnknownResource),
        }
    }

    fn get_response(&mut self) -> Result<TodoistResponse, DothisError> {
        let resource_types = self.get_resource_types()?;

        match self.client.get_resources(resource_types) {
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

pub struct AddCommand {
    client: TodoistClient,
    resource_type: String,
    new_resource: NewProject,
}

impl AddCommand {
    pub fn new(
        token: &str,
        resource_type: &str,
        name: &str,
        color: Option<&str>,
        child_order: Option<u32>,
        is_favorite: Option<bool>,
    ) -> AddCommand {
        let project = NewProject::new(name, color, child_order, is_favorite);
        AddCommand {
            client: TodoistClient::new(token),
            new_resource: project,
            resource_type: resource_type.to_owned(),
        }
    }
}

impl Command for AddCommand {
    fn execute(&mut self) -> Result<(), DothisError> {
        let commands = vec![TodoistCommand::new(&self.new_resource, None)];
        match self
            .client
            .sync_commands(vec!["projects".to_string()], None, commands)
        {
            Ok(response) => Ok(()),
            Err(e) => Err(DothisError::ApiError(e)),
        }
    }
}
