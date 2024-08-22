use clap::Command;

pub mod task;

pub fn run(mut command: Command) {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("title")
                .expect("The argument `title` was not provided");
            let description = sub_matches.get_one::<String>("description");

            todo!();
        }
        Some(("mark", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("title")
                .expect("The argument `title` was not provided");
            let as_undone = sub_matches.get_flag("as_done");

            todo!();
        },
        _ => {
            let _ = command.print_help();
        }
    }
}
