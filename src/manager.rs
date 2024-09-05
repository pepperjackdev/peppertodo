use std::error::Error;

pub mod task;

use rusqlite::{params, Connection, Statement};
use task::{Task, TaskStatus};

pub struct TaskManager<'a> {
    connection: &'a Connection,
}

impl<'a> TaskManager<'a> {
    pub fn new(connection: &Connection) -> TaskManager {
        // initializing, if not present, the working table for TaskManager
        let _ = connection.execute(
            r#"CREATE TABLE IF NOT EXISTS tasks ("id" INTEGER PRIMARY KEY AUTOINCREMENT, "title" text, "description" text, "status" text)"#, ()
        );

        TaskManager { connection }
    }

    pub fn add_new_task(&mut self, title: &str, description: &str) -> Result<(), Box<dyn Error>> {

        // checking for title availability
        if let Ok(_) = self.get_task(title) {
            return Err(Box::<dyn std::error::Error>::from("a task with the same title already exists"))
        }

        let result = self.connection.execute(
            r#"INSERT INTO "tasks" ("title", "description", "status") VALUES (?1, ?2, ?3)"#,
            params![title, description, TaskStatus::Undone],
        );

        Ok(result.map(|_| ())?)
    }

    pub fn get_all_tasks(&self, filter: Option<&TaskStatus>) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut stmt: Statement<'_>;

        let mut rows = match filter {
            Some(filter) => {
                stmt = self
                    .connection
                    .prepare(r#"SELECT * FROM "tasks" WHERE "status"=?1"#)?;
                stmt.query(params![filter])?
            }
            None => {
                stmt = self.connection.prepare(r#"SELECT * FROM "tasks""#)?;
                stmt.query([])?
            }
        };

        let mut tasks = Vec::new();

        loop {
            let row = rows.next()?;
            match row {
                Some(task) => tasks.push(Task::from(self.connection, task.get("id")?)),
                None => break,
            }
        }

        Ok(tasks)
    }

    pub fn get_task(&self, title: &str) -> Result<Task, Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"SELECT * FROM "tasks" WHERE "title"=?1"#)?;
        let mut result = stmt.query(params![title])?;
        let rows = result.next()?;
        match rows {
            Some(task) => Ok(Task::from(self.connection, task.get("id")?)),
            None => Err(Box::<dyn std::error::Error>::from("No task found")),
        }
    }

    pub fn delete_task(&mut self, title: &str) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"DELETE FROM "tasks" WHERE "title"=?1"#)?;
        let result = stmt.execute(params![title]);
        Ok(result.map(|_| ()).map_err(Box::new)?)
    }
}
