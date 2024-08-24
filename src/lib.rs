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
            let task = manager.get_task_mut(title)?;
            task.set_status(status);
            Ok(())
        }
        Some(("edit", sub_matches)) => {
            let target_title = sub_matches.get_one::<String>("target").expect("Argument marked as required");

            let new_title = sub_matches.get_one::<String>("title");
            let new_description = sub_matches.get_one::<String>("description");

            let task = manager.get_task_mut(&target_title)?;

            if let Some(title) = new_title {
                task.set_title(title);
            }

            if let Some(description) = new_description {
                task.set_description(Some(description));
            }

            Ok(())
        }
        Some(("delete", sub_matches)) => {
            let title = sub_matches.get_one::<String>("delete").expect("Argument marked as required");
            manager.delete_task(&title);
            Ok(())
        }
        _ => {
            let _ = command.print_help();
            Ok(())
        }
    }
}
