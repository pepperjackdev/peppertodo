use clap::{command, Arg, ArgAction, Command};
use todo::{manager::TaskManager, run};

fn main() {
    let mut manager = TaskManager::new();
    let command: Command = command!()
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
            Command::new("mark")
                .about("Marks a task as done (or undone)")
                .arg(
                    Arg::new("as_done")
                        .short('u')
                        .long("undone")
                        .help("Marks a done task as undone")
                        .action(ArgAction::SetFalse)   
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .help("The task title")
                        .required(true) 
                )
        );

    run(&mut manager, command);}
