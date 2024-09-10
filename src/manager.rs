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
            r#"CREATE TABLE IF NOT EXISTS tasks (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT, 
                "title" text, 
                "description" text, 
                "status" text
            )"#, ()
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
                    .prepare(
                        r#"SELECT "id" FROM "tasks" WHERE "status"=?1"#)?;
                stmt.query(params![filter])?
            }
            None => {
                stmt = self.connection.prepare(
                    r#"SELECT "id"
                        FROM "tasks"
                        ORDER BY "status",
	                        CASE
		                        WHEN "status" = "undone" THEN 2
		                        WHEN "status" = "underway" THEN 1
		                        WHEN "status" = "done" THEN 3
	                        END;"#)?;
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

    pub fn clear_done_tasks(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"DELETE FROM "tasks" WHERE "status" = ?1"#)?;
        let result = stmt.execute(params![TaskStatus::Done]);
        Ok(result.map(|_| ()).map_err(Box::new)?)
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::{task::TaskStatus, TaskManager};

    #[test]
    fn test_add_new_task() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);
        let _ = manager.add_new_task("task title", "task description");
        assert!(!manager.get_all_tasks(None).unwrap().is_empty());
    }

    #[test]
    fn test_add_new_task_with_already_taken_title() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);
        let _ = manager.add_new_task("task title", "task description");
        let result = manager.add_new_task("task title", "another t. description");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_tasks() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);
        let _ = manager.add_new_task("task title", "task description");
        let result = manager.get_all_tasks(None);
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_get_all_tasks_filtered() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);
        let _ = manager.add_new_task("first task", "task description");
        let _ = manager.add_new_task("second task", "another task");
        let _ = manager.get_task("first task").unwrap().set_status(&TaskStatus::Underway);
        assert_eq!(1, manager.get_all_tasks(Some(&TaskStatus::Undone)).unwrap().len());
    }

    #[test]
    fn test_get_task() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);
        let _ = manager.add_new_task("task title", "task description");
        assert!(manager.get_task("task title").is_ok())
    }

    #[test]
    fn test_delete_task() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);
        let _ = manager.add_new_task("task title", "task description");
        let _ = manager.delete_task("task title");
        assert!(manager.get_all_tasks(None).unwrap().is_empty())
    }

    #[test]
    fn test_clear_done_tasks() {
        let conn = Connection::open_in_memory().unwrap();
        let mut manager = TaskManager::new(&conn);

        // adding some tasks
        let _ = manager.add_new_task("task A", "desc A");
        let _ = manager.add_new_task("task B", "desc B");
        let _ = manager.get_task("task A").unwrap().set_status(&TaskStatus::Done);

        let _ = manager.clear_done_tasks();

        assert_eq!(1, manager.get_all_tasks(None).unwrap().len());
    }
}