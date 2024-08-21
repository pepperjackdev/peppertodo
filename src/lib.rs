use clap::Command;
use manager::TaskManager;
use task::Task;

pub mod manager;
pub mod task;

pub fn run(manager: &mut TaskManager, mut command: Command) {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("title")
                .expect("The argument `title` was not provided");
            let description = sub_matches.get_one::<String>("description");

            match description {
                Some(desc) => manager.add_new_task(Task::new(title, Some(&desc))),
                None => manager.add_new_task(Task::new(title, None)),
            }
        }
        Some(("mark", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title")
                .expect("The argument `title` was not provided");
            let as_undone = sub_matches.get_flag("as_done");
        }
        _ => {
            command.print_help();
        }
    }
}
