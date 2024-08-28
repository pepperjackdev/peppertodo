use std::error::Error;

pub mod task;

use rusqlite::{params, Connection};
use task::{Task, TaskStatus};

pub struct TaskManager {
    connection: Connection,
}

impl TaskManager {
    pub fn new(path: &str) -> Result<TaskManager, Box<dyn Error>> {
        let connection = Connection::open(path)?;

        // initializing task table if not present
        let _ = connection.execute("CREATE TABLE IF NOT EXISTS tasks ('id' INTEGER PRIMARY KEY AUTOINCREMENT, 'title' text, 'description' text, 'status' text)", ());

        Ok(TaskManager { connection })
    }

    pub fn add_new_task(
        &mut self,
        title: &str,
        description: &str,
    ) -> Result<(), Box<dyn Error>> {
        let result = self.connection.execute(
            r#"INSERT INTO "tasks" ("title", "description", "status") VALUES (?1, ?2, ?3)"#,
            params![title, description, TaskStatus::UNDONE],
        );

        Ok(result.map(|_| ())?)
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare(r#"SELECT * FROM "tasks""#)?;
        let mut rows = stmt.query([])?;

        let mut tasks = Vec::new();

        loop {
            let row = rows.next()?;
            match row {
                Some(task) => {
                    tasks.push(Task::from(&self.connection, task.get("id")?))
                },
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
            Some(task) => Ok(Task::from(&self.connection, task.get("id")?)),
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
