use std::env;
use std::fmt;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use todoist::api::client::TodoistClient;
use todoist::api::resource::{Filter, Item, Label, Note, Project, ProjectNote, Reminder};

fn main() {
    let matches = App::new("Todoist CLI")
        .version("1.0")
        .author("Tomas Farias")
        .about("CLI for Todoist")
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
            Arg::with_name("resource")
                .takes_value(true)
                .index(1)
                .help("")
        )
        .get_matches();

    let token = matches.value_of("token").unwrap();
    let client = TodoistClient::new(&token);

    let mut table = Table::new();
    let tbl_format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
            &[format::LinePosition::Title],
            format::LineSeparator::new('_', ' ', ' ', ' '),
        )
        .build();
    table.set_format(tbl_format);

    match matches.value_of("resource") {
        Some("project") | Some("projects") => {
            let resource_types = vec!["projects".to_owned()];
            let response = client.get_resource(resource_types).unwrap();

            if let Some(projects) = response.projects {
                table.set_titles(row!["Project"]);

                for project in projects {
                    table.add_row(row![project.name]);
                }

                table.printstd();
            } else {
                println!("No projects found")
            }
        }
        Some("task") | Some("tasks") | None => {
            let resource_types = vec!["items".to_owned(), "projects".to_owned()];
            let response = client.get_resource(resource_types).unwrap();

            if let Some(items) = response.items {
                table.set_titles(row!["Project", "Due", "Content"]);

                for project in response.projects.unwrap().iter() {
                    for task in items.iter() {
                        if task.project_id == project.id {
                            table.add_row(row![project.name, task.due.string, task.content]);
                        }
                    }
                }

                table.printstd();
            } else {
                println!("No tasks found");
            }
        }
        Some("labels") | Some("label") => {
            let resource_types = vec!["labels".to_owned()];
            println!("{:?}", client.get_resource(resource_types).unwrap());
        }
        Some("note") | Some("notes") => {
            let resource_types = vec!["notes".to_owned()];
            println!("{:?}", client.get_resource(resource_types).unwrap());
        }
        Some(other) => {
            println!("error: invalid resource {:?}", other);
        }
    }
}
