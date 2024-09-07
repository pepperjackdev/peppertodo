use std::{error::Error, fs, path::PathBuf};

use cli::{Cli, Commands};
use manager::TaskManager;

pub mod cli;
pub mod manager;

pub fn run(cli: &Cli, manager: &mut TaskManager) -> Result<(), Box<dyn Error>> {
    match &cli.command {
        Commands::Add { title, description } => {
            manager.add_new_task(title, description)
        },
        Commands::List { filter } => {
            manager.get_all_tasks(filter.as_ref())?.iter()
                .for_each(|task| println!("{task}"));
            Ok(())
        },
        Commands::Mark { target, status } => {
            let mut task = manager.get_task(target)?;
            task.set_status(status)
        },
        Commands::Edit { target, title, description } => {
            let mut task = manager.get_task(target)?;

            // Editing the title if provided
            if let Some(new_title) = title {
                task.set_title(new_title)?;
            };

            // Editing the description if provided
            if let Some(new_description) = description {
                task.set_description(new_description)?;
            };

            // if nothing bad has happened...
            Ok(())
        },
        Commands::Delete { target } => {
            manager.delete_task(target)
        },
        Commands::Clear => {
            manager.clear_done_tasks()
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
