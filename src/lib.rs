use clap::Command;

pub mod task;

pub fn run(command: &mut Command) {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title")
                .expect("Safe unwrap: 'title' is required");

            let description = sub_matches.get_one::<String>("description");
        }
        _ => {
            let _ = command.print_help();
        }
    }
}
