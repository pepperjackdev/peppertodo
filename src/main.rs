use clap::{builder::EnumValueParser, command, Arg, Command};
use todo::{manager::TaskManager, task::{Task, TaskStatus}};

fn main() {
    let mut manager = TaskManager::new();
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
                    .long("description"),
            ),
        )
        .subcommand(
            Command::new("list")
                .about("Displays all the tasks")
                .arg(
                    Arg::new("filter")
                        .value_parser(EnumValueParser::<TaskStatus>::new())
                )
        )
        .subcommand(
            Command::new("mark")
                .about("Marks a task's status")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("The title of task to mark")
                        .required(true)
                )
                .arg(
                    Arg::new("status")
                        .value_parser(EnumValueParser::<TaskStatus>::new())
                        .required(true)
                )
        );

    if let Err(error) = todo::run(&mut command, &mut manager) {
        eprintln!("{error}");
    }
}
