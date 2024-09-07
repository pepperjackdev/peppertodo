use std::error::Error;

use peppertodo::{cli::{Cli, Commands}, manager::{task::{Task, TaskStatus}, TaskManager}};
use rusqlite::Connection;


#[test]
fn test_run_add() -> Result<(), Box<dyn Error>> {
    // setting up the manager
    let conn = Connection::open_in_memory()?;
    let mut manager = TaskManager::new(&conn);

    // setting up the cli
    let cli = Cli::from(Cli { 
        command: Commands::Add { 
            title: "task title".to_string(), 
            description: "task description".to_string() 
        } 
    });

    let _ = peppertodo::run(&cli, &mut manager)?;

    assert!(!manager.get_all_tasks(None).unwrap().is_empty());

    // Well, nothing bad has happened
    Ok(())
}

#[test]
fn test_run_list() -> Result<(), Box<dyn Error>> {
    // setting up the manager
    let conn = Connection::open_in_memory()?;
    let mut manager = TaskManager::new(&conn);

    // setting up the cli
    let cli = Cli::from(Cli {
        command: Commands::List { filter: None }
    });

    let _ = peppertodo::run(&cli, &mut manager)?;

    Ok(())
}

#[test]
fn test_run_mark() -> Result<(), Box<dyn Error>> {
    // setting up the manager
    let conn = Connection::open_in_memory()?;
    let mut manager = TaskManager::new(&conn);

    // adding a task 
    manager.add_new_task("task title", "task description")?;

    // setting up the cli
    let cli = Cli::from(Cli {
        command: Commands::Mark { 
            target: "task title".to_string(), 
            status: TaskStatus::Done }
    });

    // setting up the cli
    let _ = peppertodo::run(&cli, &mut manager);

    assert!(!manager.get_all_tasks(Some(&TaskStatus::Done)).unwrap().is_empty());

    Ok(())
}

#[test]
fn test_run_edit() -> Result<(), Box<dyn Error>> {
    // setting up the manager
    let conn = Connection::open_in_memory()?;
    let mut manager = TaskManager::new(&conn);

    // adding a task
    manager.add_new_task("task title", "task description")?;

    // setting up the cli
    let cli = Cli::from(Cli {
        command: Commands::Edit { 
            target: "task title".to_string(), 
            title: Some("new title".to_string()), 
            description: Some("new description".to_string())  
        }
    });

    // running the code to test
    peppertodo::run(&cli, &mut manager)?;

    // assetions
    let task = Task::from(&conn, 1); // manually instantiated task
    assert_eq!("new title", task.get_title().unwrap());
    assert_eq!("new description", task.get_description().unwrap());

    Ok(())
}

#[test]
fn test_run_delete() -> Result<(), Box<dyn Error>> {
    // setting up the manager
    let conn = Connection::open_in_memory()?;
    let mut manager = TaskManager::new(&conn);

    // populating the db 
    manager.add_new_task("task title", "task description")?;

    // setting up the cli
    let cli = Cli::from(Cli { 
        command: Commands::Delete { 
            target: "task title".to_string(), 
        } 
    });

    let _ = peppertodo::run(&cli, &mut manager)?;

    assert!(manager.get_all_tasks(None).unwrap().is_empty());

    // Well, nothing bad has happened
    Ok(())
}

#[test]
fn test_run_clear() -> Result<(), Box<dyn Error>> {
    // setting up the manager
    let conn = Connection::open_in_memory()?;
    let mut manager = TaskManager::new(&conn);

    // populating the db 
    manager.add_new_task("task", "desc")?;
    manager.get_task("task").unwrap().set_status(&TaskStatus::Done)?;

    // setting up the cli
    let cli = Cli::from(Cli { 
        command: Commands::Clear
    });

    let _ = peppertodo::run(&cli, &mut manager)?;

    assert!(manager.get_all_tasks(None).unwrap().is_empty());

    // Well, nothing bad has happened
    Ok(())
}