use clap::Command;
use manager::TaskManager;
use task::{Task, TaskStatus};

pub mod manager;
pub mod task;

pub fn run(
    command: &mut Command,
    manager: &mut TaskManager,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        // Adding tasks
        Some(("add", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("title")
                .expect("Safe unwrap: 'title' is required");

            let description = sub_matches.get_one::<String>("description");

            let task_to_add = Task::from(title, description.map(|desc| desc.as_str()));

            let _ = manager.add_new_task(task_to_add);
            Ok(())
        }

        // Viewing tasks
        Some(("list", sub_matches)) => {
            let filter = sub_matches.get_one::<TaskStatus>("filter");

            match filter {
                Some(status) => {
                    manager
                        .get_all_tasks()
                        .unwrap()
                        .iter()
                        .filter(|task| task.get_status() == status)
                        .for_each(|task| println!("{task}"));
                }
                None => {
                    manager
                        .get_all_tasks()
                        .unwrap()
                        .iter()
                        .for_each(|task| println!("{task}"));
                }
            }

            Ok(())
        }

        // Marking tasks
        Some(("mark", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'target' is required");

            let status = sub_matches
                .get_one::<TaskStatus>("status")
                .expect("Safe unwrap: 'status' is required");

            let mut task = manager.get_task(title)?;

            task.set_status(status);

            Ok(())
        }

        // Editing tasks
        Some(("edit", sub_matches)) => {
            let target_title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'edit' is required");

            let new_title = sub_matches.get_one::<String>("title");
            let new_description = sub_matches.get_one::<String>("description");

            let mut task = manager.get_task(&target_title)?;

            if let Some(title) = new_title {
                task.set_title(title);
            }

            if let Some(description) = new_description {
                task.set_description(Some(description));
            }

            Ok(())
        }

        // Deleting tasks
        Some(("delete", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'target' is required");

            manager.delete_task(&title);

            Ok(())
        }
        _ => {
            let _ = command.print_help();
            Ok(())
        }
    }
}
