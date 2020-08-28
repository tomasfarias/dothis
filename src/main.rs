extern crate clap;
use clap::{Arg, App, SubCommand};

mod api;
use api::client::TodoistClient;
use api::resource::{ Project };

fn main() {
    let matches = App::new("Todoist CLI")
        .version("1.0")
        .author("Tomas Farias")
        .about("CLI for Todoist")
        .arg(Arg::with_name("token")
            .required(true)
            .takes_value(true)
            .long("token")
            .help("Todoist token"))
        .subcommand(SubCommand::with_name("get")
            .about("get a resource")
            .arg(Arg::with_name("resource")
                .required(true)
                .help("resource to get"))
            .arg(Arg::with_name("from")
                .short("f")
                .help("filter by due date from inclusive"))
            .arg(Arg::with_name("to")
                .short("t")
                .help("filter by due date to inclusive")))
        .get_matches();

    let token = matches.value_of("token").unwrap();

    match matches.subcommand_name() {
        Some("get") => {
            let client = TodoistClient::new(token);
            let resource_types = vec!["projects".to_owned()];
            let project = client.get_resource(resource_types);
            println!("{:?}", project);
        },
        None => println!("No subcommand was used"),
        _ => println!("Should never get here"),
    }
}
