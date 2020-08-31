use std::env;

extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

use todoist::api::client::TodoistClient;
use todoist::api::resource::{Item, Label, Note, Project};

fn main() {
    let matches = App::new("Todoist CLI")
        .version("1.0")
        .author("Tomas Farias")
        .about("CLI for Todoist")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("token")
                .required(false)
                .takes_value(true)
                .long("token")
                .help("Todoist token"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get a resource")
                .arg(
                    Arg::with_name("resource")
                        .required(true)
                        .help("resource to get"),
                )
                .arg(
                    Arg::with_name("from")
                        .short("f")
                        .help("filter by due date from inclusive"),
                )
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .help("filter by due date to inclusive"),
                ),
        )
        .get_matches();

    let token = match matches.value_of("token") {
        Some(t) => t.to_string(),
        None => {
            if env::var("TODOIST_API_TOKEN").is_err() {
                println!("error: Required Todoist API token was not provided");
                return;
            };
            env::var("TODOIST_API_TOKEN").unwrap().to_string()
        }
    };

    match matches.subcommand_name() {
        Some("get") => {
            let client = TodoistClient::new(&token);
            let resource = matches
                .subcommand_matches("get")
                .unwrap()
                .value_of("resource")
                .unwrap();
            match resource {
                "project" | "projects" => {
                    let resource_types = vec!["projects".to_owned()];
                    println!("{:?}", client.get_resource(resource_types).unwrap());
                    return;
                }
                "task" | "tasks" => {
                    let resource_types = vec!["items".to_owned()];
                    println!("{:?}", client.get_resource(resource_types).unwrap());
                }
                "labels" | "label" => {
                    let resource_types = vec!["labels".to_owned()];
                    println!("{:?}", client.get_resource(resource_types).unwrap());
                }
                "note" | "notes" => {
                    let resource_types = vec!["notes".to_owned()];
                    println!("{:?}", client.get_resource(resource_types).unwrap());
                }
                _ => {
                    println!("error: invalid resource {:?}", resource);
                }
            }
        }
        Some(other) => {
            println!("error: Invalid subcommand {}", other);
        }
        None => println!("error: No subcommand was used"),
    }
}
