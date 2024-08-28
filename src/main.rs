use clap::{builder::EnumValueParser, command, Arg, Command};
use todo::manager::{task::TaskStatus, TaskManager};

fn main() {
    // Setting up the task manager
    let mut manager = TaskManager::new(r#"appdata.db"#).expect("Problem accessing the database");

    // Setting up the command line options
    let mut command: Command = command!()
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("The title of the task")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .help("The description of the task (optional)")
                        .short('d')
                        .long("description")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Displays all the tasks")
                .arg(Arg::new("filter").value_parser(EnumValueParser::<TaskStatus>::new())),
        )
        .subcommand(
            Command::new("mark")
                .about("Marks a task's status")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("title")
                        .help("The title of the task to mark")
                        .required(true),
                )
                .arg(
                    Arg::new("status")
                        .value_parser(EnumValueParser::<TaskStatus>::new())
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("edit")
                .about("Modifies a task's title or description")
                .arg(
                    Arg::new("target")
                        .long("target")
                        .help("The title of the task to edit")
                        .required(true),
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("The new title of the task"),
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .help("The new description of the task"),
                ),
        )
        .subcommand(
            Command::new("delete").arg(
                Arg::new("target")
                    .short('t')
                    .long("title")
                    .help("Deletes the task with the provided title")
                    .required(true),
            ),
        );

    if let Err(error) = todo::run(&mut command, &mut manager) {
        eprintln!("{error}");
    }
}
