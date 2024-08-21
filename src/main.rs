use clap::{command, Arg, Command};
use todo::{manager::TaskManager, run};

fn main() {
    let manager = TaskManager::new();
    let command = command!().subcommand(
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
    );

    run(manager, command);
}
