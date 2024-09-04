use std::{error::Error, fs, io::ErrorKind, path::PathBuf};

use clap::Command;
use manager::{task::TaskStatus, TaskManager};

pub mod manager;

pub fn run(command: &mut Command, manager: &mut TaskManager) -> Result<(), Box<dyn Error>> {
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("title")
                .expect("Safe unwrap: 'title' is required");

            let description = sub_matches
                .get_one::<String>("description")
                .expect("Safe unwrap: 'description' is required");

            manager.add_new_task(title, description)
        }
        Some(("list", sub_matches)) => {
            let filter = sub_matches.get_one::<TaskStatus>("filter");
            manager.get_all_tasks(filter)?
                .into_iter()
                .for_each(|task| println!("{task}"));
            Ok(())
        }
        Some(("mark", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'target' is required");

            let status = sub_matches
                .get_one::<TaskStatus>("status")
                .expect("Safe unwrap: 'status' is required");

            let task = manager.get_task(title);

            if let Ok(mut task) = task {
                let _ = task.set_status(status);
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

            let task = manager.get_task(target_title);

            match task {
                Ok(mut task_to_edit) => {
                    if let Some(title) = new_title {
                        let _ = task_to_edit.set_title(title);
                    }

                    if let Some(description) = new_description {
                        let _ = task_to_edit.set_description(description);
                    }

                    Ok(())
                }
                Err(_) => Err(Box::new(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Task to edit not found",
                ))),
            }
        }
        Some(("delete", sub_matches)) => {
            let title = sub_matches
                .get_one::<String>("target")
                .expect("Safe unwrap: 'title' is required");
            manager.delete_task(title)
        }
        _ => {
            let _ = command.print_help();
            Ok(())
        }
    }
}

pub fn setup_application_directory(app_home: &str) -> PathBuf {
    let data_home = dirs::data_dir().expect("Unable to retrive system's data dir");
    let app_home = data_home.join(app_home);

    if !app_home.exists() {
        let _ = fs::create_dir_all(&app_home);
    }

    app_home
}