use std::str::FromStr;
use std::{error::Error, fmt::Display};

use clap::{builder::PossibleValue, ValueEnum};
use rusqlite::types::{FromSql, FromSqlError, ValueRef};
use rusqlite::{params, Connection, ToSql};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Undone,
    Underway,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskStatus::Undone => write!(f, "undone"),
            TaskStatus::Underway => write!(f, "underway"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undone" => Ok(TaskStatus::Undone),
            "underway" => Ok(TaskStatus::Underway),
            "done" => Ok(TaskStatus::Done),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid task status",
            ))),
        }
    }
}

impl ValueEnum for TaskStatus {
    fn value_variants<'a>() -> &'a [Self] {
        &[TaskStatus::Undone, TaskStatus::Underway, TaskStatus::Done]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(self.to_string()))
    }
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        let s: String = FromSql::column_result(value)?;
        match s.as_str() {
            "undone" => Ok(TaskStatus::Undone),
            "underway" => Ok(TaskStatus::Underway),
            "done" => Ok(TaskStatus::Done),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for TaskStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

#[derive(Debug, Clone)]
pub struct Task<'a> {
    connection: &'a Connection,
    id: i32,
}

impl<'a> Task<'a> {
    pub fn from(connection: &'a Connection, id: i32) -> Task<'a> {
        Task { connection, id }
    }

    pub fn get_title(&self) -> Result<String, Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"SELECT "title" FROM "tasks" WHERE "id"=?1"#)?;
        let mut result = stmt.query(params![&self.id])?;
        if let Some(row) = result.next()? {
            let title: String = row.get("title")?;
            Ok(title)
        } else {
            Err(Box::<dyn std::error::Error>::from("Title not found"))
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"UPDATE "tasks" SET "title" = ?1 WHERE "id" = ?2"#)?;
        let _ = stmt.execute(params![title, self.id]);
        Ok(())
    }

    pub fn get_description(&self) -> Result<String, Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"SELECT "description" FROM "tasks" WHERE "id"=?1"#)?;
        let mut result = stmt.query(params![&self.id])?;
        if let Some(row) = result.next()? {
            let title: String = row.get("description")?;
            Ok(title)
        } else {
            Err(Box::<dyn std::error::Error>::from("Description not found"))
        }
    }

    pub fn set_description(&mut self, description: &str) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"UPDATE "tasks" SET "description" = ?1 WHERE "id" = ?2"#)?;
        let _ = stmt.execute(params![description, self.id]);
        Ok(())
    }

    pub fn get_status(&self) -> Result<TaskStatus, Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"SELECT "status" FROM "tasks" WHERE "id"=?1"#)?;
        let mut result = stmt.query(params![self.id])?;
        if let Some(row) = result.next()? {
            let status: TaskStatus = row.get("status")?;
            Ok(status)
        } else {
            Err(Box::<dyn std::error::Error>::from("Status not found"))
        }
    }

    pub fn set_status(&mut self, status: &TaskStatus) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .connection
            .prepare(r#"UPDATE "tasks" SET "status" = ?1 WHERE "id" = ?2"#)?;
        let _ = stmt.execute(params![status, self.id]);
        Ok(())
    }
}

impl<'a> Display for Task<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.get_status().unwrap(),
            self.get_title().unwrap(),
            self.get_description().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::manager::{task::TaskStatus, TaskManager};

    use super::Task;

    fn set_up_connection() -> Connection {
        // initializing a connection to a database that lives on the memory
        let connection = Connection::open_in_memory()
            .expect("unable to create in-memory database for testing purpuses");

        // setting up a temporary TaskManager to initialized tasks table
        let mut manager = TaskManager::new(&connection); // the DB has been initialized with "tasks" table
        let _ = manager.add_new_task("task title", "task description"); // added a test task

        connection
    }

    fn set_up_task(connection: &Connection) -> Task {
        Task::from(&connection, 1) // as this is the only task who lives on the db, its id will be `1`
    }

    #[test]
    fn test_get_title() {
        let conn = set_up_connection();
        let task = set_up_task(&conn);
        assert_eq!("task title", task.get_title().unwrap());
    }

    #[test]
    fn test_set_title() {
        let conn = set_up_connection();
        let mut task = set_up_task(&conn);
        task.set_title("new task title")
            .expect("Task should exists and database should be accessible");
        assert_eq!("new task title", task.get_title().unwrap())
    }

    #[test]
    fn test_get_description() {
        let conn = set_up_connection();
        let task = set_up_task(&conn);
        assert_eq!("task description", task.get_description().unwrap());
    }

    #[test]
    fn test_set_description() {
        let conn = set_up_connection();
        let mut task = set_up_task(&conn);
        task.set_title("new task description")
            .expect("Task should exists and database should be accessible");
        assert_eq!("new task description", task.get_title().unwrap())
    }

    #[test]
    fn test_get_status() {
        let conn = set_up_connection();
        let task = set_up_task(&conn);
        assert_eq!(TaskStatus::Undone, task.get_status().unwrap())
    }

    #[test]
    fn test_set_status() {
        let conn = set_up_connection();
        let mut task = set_up_task(&conn);
        task.set_status(&TaskStatus::Underway)
            .expect("Task should exists and database should be accessible");
        assert_eq!(TaskStatus::Underway, task.get_status().unwrap())
    }
}