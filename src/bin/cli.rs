use std::collections::HashMap;
use std::env;
use std::error;
use std::fmt;
use std::process;

extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use dothis::api::client::{TodoistApiError, TodoistClient};
use dothis::api::resource::{Filter, Item, Label, Note, Project, ProjectNote, Reminder};

fn main() {
    let matches = App::new("dothis")
        .version("1.0")
        .author("Tomas Farias")
        .about("dothis the CLI for Todoist")
        .arg(
            Arg::with_name("token")
                .required(true)
                .takes_value(true)
                .long("token")
                .short("t")
                .env("TODOIST_API_TOKEN")
                .help("Todoist API token, not required if TODOIST_API_TOKEN environment variable is set")
        )
        .arg(
            Arg::with_name("command")
                .takes_value(true)
                .default_value("list")
		.alias("ls")
                .help("command to execute")
        )
        .arg(
            Arg::with_name("resource")
                .takes_value(true)
                .default_value("tasks")
                .help("resource to operate on")
        )
        .get_matches();

    let dothis = Dothis::new(matches);

    process::exit(match dothis.run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            i32::from(err)
        }
    })
}

struct Dothis<'a> {
    args: ArgMatches<'a>,
}

#[derive(Debug)]
enum DothisError {
    ApiError(TodoistApiError),
    EmptyResponseError,
    UnknownResource,
    MissingCommand,
    UnknownCommand,
}

impl error::Error for DothisError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DothisError::ApiError(ref err) => Some(err),
            DothisError::EmptyResponseError => None,
            DothisError::UnknownResource => None,
            DothisError::MissingCommand => None,
            DothisError::UnknownCommand => None,
        }
    }
}

impl fmt::Display for DothisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DothisError::ApiError(ref err) => err.fmt(f),
            DothisError::MissingCommand => write!(f, "missing command"),
            // These three should specify what is unknown or what
            // field was empty. Also, I am not too fond of "resource" as a
            // word for tasks, projects, notes, and other todoist entities.
            DothisError::UnknownCommand => write!(f, "unknown command"),
            DothisError::EmptyResponseError => write!(f, "no resources found"),
            DothisError::UnknownResource => write!(f, "unknown resource"),
        }
    }
}

impl From<TodoistApiError> for DothisError {
    fn from(err: TodoistApiError) -> DothisError {
        DothisError::ApiError(err)
    }
}

impl From<DothisError> for i32 {
    fn from(err: DothisError) -> Self {
        match err {
            DothisError::ApiError(ref err) => 69,
            DothisError::EmptyResponseError => 69,
            DothisError::UnknownResource => 64,
            DothisError::MissingCommand => 64,
            DothisError::UnknownCommand => 64,
        }
    }
}

impl Dothis<'_> {
    fn new(args: ArgMatches<'static>) -> Dothis {
        Dothis { args: args }
    }

    fn run(&self) -> Result<(), DothisError> {
        match self.args.value_of("command") {
            Some("list") => self.run_list(),
            Some(other) => Err(DothisError::UnknownCommand),
            // Should never get here since command is required.
            // Wondering if this is even necessary...
            None => Err(DothisError::MissingCommand),
        }
    }

    fn run_list(&self) -> Result<(), DothisError> {
        let mut table = Table::new();
        let tbl_format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Title],
                format::LineSeparator::new('=', ' ', ' ', ' '),
            )
            .build();
        table.set_format(tbl_format);

        let token = self.args.value_of("token").unwrap();

        match self.args.value_of("resource") {
            Some("tasks") | Some("task") | None => TaskManager::new(&token).list(table),
            Some("projects") | Some("project") => ProjectManager::new(&token).list(table),
            Some("labels") | Some("label") => LabelsManager::new(&token).list(table),
            Some("notes") | Some("note") => NotesManager::new(&token).list(table),
            other => Err(DothisError::UnknownResource),
        }
    }
}

trait Manager {
    // list command should eventually support other outputs besides stdout via an argument
    fn list(&self, table: Table) -> Result<(), DothisError>;
}

struct TaskManager {
    client: TodoistClient,
}

impl TaskManager {
    fn new(token: &str) -> TaskManager {
        TaskManager {
            client: TodoistClient::new(token),
        }
    }
}

impl Manager for TaskManager {
    fn list(&self, mut table: Table) -> Result<(), DothisError> {
        let resource_types = vec!["items".to_string(), "projects".to_string()];
        match self.client.get_resource(resource_types) {
            Ok(response) => {
                let tasks = match response.items {
                    Some(tasks) => tasks,
                    None => return Err(DothisError::EmptyResponseError),
                };
                let projects = match response.projects {
                    Some(projects) => projects,
                    None => return Err(DothisError::EmptyResponseError),
                };

                table.set_titles(row!["Project", "Added", "Due", "Content"]);
                for project in projects.iter() {
                    for task in tasks.iter().filter(|t| t.project_id == project.id) {
                        table.add_row(row![
                            project.name,
                            task.date_added,
                            task.due.string,
                            task.content
                        ]);
                    }
                }

                table.printstd();
                Ok(())
            }
            Err(err) => Err(DothisError::ApiError(err)),
        }
    }
}

struct ProjectManager {
    client: TodoistClient,
}

impl ProjectManager {
    fn new(token: &str) -> ProjectManager {
        ProjectManager {
            client: TodoistClient::new(token),
        }
    }
}

impl Manager for ProjectManager {
    fn list(&self, mut table: Table) -> Result<(), DothisError> {
        let resource_types = vec!["projects".to_string()];
        match self.client.get_resource(resource_types) {
            Ok(response) => {
                let projects = match response.projects {
                    Some(projects) => projects,
                    None => return Err(DothisError::EmptyResponseError),
                };

                table.set_titles(row!["Project", "Parent"]);

                let mut projects_map: HashMap<u32, Project> = HashMap::new();
                projects_map.extend(projects.into_iter().map(|p| (p.id, p)));

                for (_, project) in projects_map.iter() {
                    match project.parent_id {
                        Some(parent_id) => table.add_row(row![
                            project.name,
                            projects_map.get(&parent_id).map_or("", |p| &p.name)
                        ]),
                        None => table.add_row(row![project.name, ""]),
                    };
                }

                table.printstd();
                Ok(())
            }
            Err(err) => Err(DothisError::ApiError(err)),
        }
    }
}

struct LabelsManager {
    client: TodoistClient,
}

impl LabelsManager {
    fn new(token: &str) -> NotesManager {
        NotesManager {
            client: TodoistClient::new(token),
        }
    }
}

impl Manager for LabelsManager {
    fn list(&self, mut table: Table) -> Result<(), DothisError> {
        let resource_types = vec!["labels".to_owned()];
        match self.client.get_resource(resource_types) {
            Ok(response) => {
                let labels = match response.labels {
                    Some(labels) => labels,
                    None => return Err(DothisError::EmptyResponseError),
                };

                table.set_titles(row!["Name", "Favorite", "Deleted"]);

                for label in labels.iter() {
                    table.add_row(row![label.name, label.is_favorite, label.is_deleted]);
                }

                table.printstd();
                Ok(())
            }
            Err(err) => Err(DothisError::ApiError(err)),
        }
    }
}

struct NotesManager {
    client: TodoistClient,
}

impl NotesManager {
    fn new(token: &str) -> NotesManager {
        NotesManager {
            client: TodoistClient::new(token),
        }
    }
}

impl Manager for NotesManager {
    fn list(&self, mut table: Table) -> Result<(), DothisError> {
        let resource_types = vec![
            "notes".to_owned(),
            "projects".to_owned(),
            "items".to_owned(),
        ];
        match self.client.get_resource(resource_types) {
            Ok(response) => {
                table.set_titles(row!["Project", "Task", "Content"]);
                let tasks = match response.items {
                    Some(tasks) => tasks,
                    None => return Err(DothisError::EmptyResponseError),
                };

                let projects = match response.projects {
                    Some(projects) => projects,
                    None => return Err(DothisError::EmptyResponseError),
                };

                let notes = match response.notes {
                    Some(notes) => notes,
                    None => return Err(DothisError::EmptyResponseError),
                };

                // There has to be a better way to do this
                for note in notes.iter() {
                    for task in tasks.iter() {
                        for project in projects.iter() {
                            if note.project_id == project.id && note.item_id == task.id {
                                table.add_row(row![project.name, task.id, note.content]);
                            }
                        }
                    }
                }

                table.printstd();
                Ok(())
            }
            Err(err) => Err(DothisError::ApiError(err)),
        }
    }
}
