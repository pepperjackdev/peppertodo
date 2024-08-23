use clap::Command;
use manager::TaskManager;
use task::{Task, TaskStatus};

pub mod task;
pub mod manager;

pub fn run(command: &mut Command, manager: &mut TaskManager) -> Result<(), Box<&'static str>> {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title")
                .expect("Safe unwrap: 'title' is required");

            let description = sub_matches.get_one::<String>("description");

            let task_to_add = Task::from(title, description.map(|desc| desc.as_str()));

            manager.add_new_task(task_to_add)
        },
        Some(("list", sub_matches)) => {
            let filter = sub_matches.get_one::<TaskStatus>("filter");

            match filter {
                Some(status_filter) => {
                    manager.get_all_tasks().iter().filter(|task| task.get_status() == status_filter)
                        .for_each(|task| println!("{task}"));
                }
                None => {
                    manager.get_all_tasks().iter().for_each(|task| println!("{task}"));
                }
            }

            Ok(())
        }
        Some(("mark", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").expect("Argument marked as required");
            let status = sub_matches.get_one::<TaskStatus>("status").expect("Argument marked as required");
            manager.mark_task(title, status)
        }
        _ => {
            let _ = command.print_help();
            Ok(())
        }
    }
}
