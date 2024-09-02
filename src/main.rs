use clap::{builder::EnumValueParser, command, Arg, Command};
use peppertodo::manager::{task::TaskStatus, TaskManager};

fn main() {
    // Setting up the task manager
    let mut manager = TaskManager::new("appdata.db").expect("Problem accessing the database");

    // Setting up the command line options
    let mut command: Command = command!()
        .subcommand(
            Command::new("add")
                .alias("+")
                .about("Adds a new task")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("The title of the task")
                        .required(true)
                )
                .arg(
                    Arg::new("description")
                        .help("The description of the task (optional)")
                        .short('d')
                        .long("description")
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("Displays all the tasks")
                .arg(Arg::new("filter").value_parser(EnumValueParser::<TaskStatus>::new())),
        )
        .subcommand(
            Command::new("mark")
                .alias("!")
                .about("Marks a task's status")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("title")
                        .help("The title of the task to mark")
                        .required(true)
                )
                .arg(
                    Arg::new("status")
                        .value_parser(EnumValueParser::<TaskStatus>::new())
                        .required(true)
                ),
        )
        .subcommand(
            Command::new("edit")
                .alias("ed")
                .about("Modifies a task's title or description")
                .arg(
                    Arg::new("target")
                        .long("target")
                        .help("The title of the task to edit")
                        .required(true)
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("The new title of the task")
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .help("The new description of the task")
                ),
        )
        .subcommand(
            Command::new("delete")
                .alias("del")
                .about("Deletes the task with the provided title")
                .arg(Arg::new("target").short('t').long("title").required(true)),
        );

    if let Err(error) = peppertodo::run(&mut command, &mut manager) {
        eprintln!("{error}");
    }
}
