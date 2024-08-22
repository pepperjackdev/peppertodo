use clap::{command, Arg, Command};
use todo::run;

fn main() {
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
        );

    run(&mut command);
}
