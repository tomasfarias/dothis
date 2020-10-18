use std::process;

extern crate clap;
use clap::{App, Arg, ArgMatches};

#[macro_use]
extern crate prettytable;
use prettytable::Table;

mod command;
use command::{AddCommand, Command, ListCommand};

mod error;
use crate::error::DothisError;

fn main() {
    env_logger::init();
    let dothis = Dothis::new();

    process::exit(match dothis.run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            i32::from(err)
        }
    })
}

struct Dothis<'a> {
    token: String,
    resource: String,
    command: String,
    args: ArgMatches<'a>,
}

impl<'a> Dothis<'a> {
    fn new() -> Self {
        let matches = App::new("dothis")
            .version("1.0")
            .author("Tomas Farias")
            .about("dothis the CLI for Todoist")
            .arg(
                Arg::with_name("token")
                    .required(true)
                    .takes_value(true)
                    .require_equals(true)
                    .long("token")
                    .short("t")
                    .env("TODOIST_API_TOKEN")
                    .help("Todoist API token, not required if TODOIST_API_TOKEN environment variable is set")
            )
            .arg(
                Arg::with_name("command")
                    .takes_value(true)
                    .index(1)
                    .required(true)
                    .default_value("list")
                    .help("command to execute")
            )
            .arg(
                Arg::with_name("resource")
                    .takes_value(true)
                    .index(2)
                    .required(true)
                    .default_value_if("command", Some("list"), "tasks")
                    .help("resource to operate on")
            )
            .arg(
                Arg::with_name("name")
                    .takes_value(true)
                    .help("resource name")
                    .requires_ifs(&[
                        ("command", "add"),
                        ("resource", "projects"),
                    ])
            )
            .arg(
                Arg::with_name("color")
                    .short("c")
                    .takes_value(true)
                    .help("resource color string")
            )
            .arg(
                Arg::with_name("favorite")
                    .long("favorite")
                    .help("mark resource as favorite")
            )
            .arg(
                Arg::with_name("content")
                    .takes_value(true)
                    .help("resource content")
                    .requires_ifs(&[
                        ("command", "add"),
                        ("resource", "tasks")
                    ])
            )
            .get_matches();

        let token = matches
            .value_of("token")
            .expect("argument token is required");
        let command = matches
            .value_of("command")
            .expect("argument command is required");
        let resource = matches
            .value_of("resource")
            .expect("argument resource is required");

        Dothis {
            token: token.to_string(),
            command: command.to_string(),
            resource: resource.to_string(),
            args: matches,
        }
    }

    fn run(&self) -> Result<(), DothisError> {
        let resource_type = self.get_resource_type()?;

        match self.command.as_str() {
            "list" => {
                let mut table = Table::new();
                let mut list = ListCommand::new(&self.token, resource_type, table);

                list.execute()?;
                Ok(())
            }
            "add" => {
                let mut add = AddCommand::new(&self.token, resource_type, self.args.clone());

                add.execute()?;
                Ok(())
            }
            other => Err(DothisError::UnknownCommand),
        }
    }

    fn get_resource_type(&self) -> Result<&str, DothisError> {
        match self.resource.as_str() {
            "tasks" | "task" => Ok("tasks"),
            "projects" | "project" => Ok("projects"),
            "labels" | "label" => Ok("labels"),
            "notes" | "note" => Ok("notes"),
            other => return Err(DothisError::UnknownResource),
        }
    }
}
