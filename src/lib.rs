use std::{error::Error, io::ErrorKind};

use clap::Command;
use manager::TaskManager;
use task::{Task, TaskStatus};

pub mod manager;
pub mod task;

pub fn run(command: &mut Command, manager: &mut TaskManager) -> Result<(), Box<dyn Error>> {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("title")
                .expect("Safe unwrap: 'title' is required");

            let description = sub_matches.get_one::<String>("description");

            let task_to_add = Task::from(title, description.map(|desc| desc.as_str()));

            manager.add_new_task(task_to_add)
        }
        Some(("list", sub_matches)) => {
            let filter = sub_matches.get_one::<TaskStatus>("filter");

            match filter {
                Some(status) => {
                    manager
                        .get_all_tasks()
                        .iter()
                        .filter(|task| task.get_status() == status)
                        .for_each(|task| println!("{task}"));
                }
                None => {
                    manager
                        .get_all_tasks()
                        .iter()
                        .for_each(|task| println!("{task}"));
                }
            }

            Ok(())
        }
        Some(("mark", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'target' is required");

            let status = sub_matches
                .get_one::<TaskStatus>("status")
                .expect("Safe unwrap: 'status' is required");

            let task = manager.get_task_mut(title);

            if let Some(task) = task {
                task.set_status(status);
            } else {
                return Err(Box::new(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Task to mark not found",
                )));
            }

            Ok(())
        }
        Some(("edit", sub_matches)) => {
            let target_title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'target' is required");

            let new_title = sub_matches.get_one::<String>("title");
            let new_description = sub_matches.get_one::<String>("description");

            let task = manager.get_task_mut(&target_title);

            match task {
                Some(task_to_edit) => {
                    if let Some(title) = new_title {
                        task_to_edit.set_title(title);
                    }

                    if let Some(description) = new_description {
                        task_to_edit.set_description(Some(&description));
                    }

                    Ok(())
                }
                None => Err(Box::new(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Task to edit not found",
                ))),
            }
        }
        Some(("delete", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'title' is required");
            manager.delete_task(&title)
        }
        _ => {
            let _ = command.print_help();
            Ok(())
        }
    }
}
