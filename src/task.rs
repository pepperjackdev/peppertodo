use std::{error::Error, fmt::Display};
use std::str::FromStr;

use clap::{builder::PossibleValue, ValueEnum};
use rusqlite::types::{FromSqlError, ValueRef};
use rusqlite::{types::FromSql, ToSql};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    UNDONE,
    UNDERWAY,
    DONE,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TaskStatus::UNDONE => write!(f, "undone"),
            TaskStatus::UNDERWAY => write!(f, "underway"),
            TaskStatus::DONE => write!(f, "done"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undone" => Ok(TaskStatus::UNDONE),
            "underway" => Ok(TaskStatus::UNDERWAY),
            "done" => Ok(TaskStatus::DONE),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid task status",
            ))),
        }
    }
}

impl ValueEnum for TaskStatus {
    fn value_variants<'a>() -> &'a [Self] {
        &[TaskStatus::UNDONE, TaskStatus::UNDERWAY, TaskStatus::DONE]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(&self.to_string()))
    }
}

impl ToSql for TaskStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        let s: String = FromSql::column_result(value)?;
        match s.as_str() {
            "undone" => Ok(TaskStatus::UNDONE),
            "underway" => Ok(TaskStatus::UNDERWAY),
            "done" => Ok(TaskStatus::DONE),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Task {
    title: String,
    description: Option<String>,
    status: TaskStatus,
}

impl Task {
    pub fn from(title: &str, description: Option<&str>) -> Task {
        let description = match description {
            Some(description) => Some(description.to_string()),
            None => None,
        };

        Task {
            title: title.to_string(),
            description: description,
            status: TaskStatus::UNDONE,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, description: Option<&str>) {
        self.description = description.map(|desc| desc.to_string());
    }

    pub fn get_status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: &TaskStatus) {
        self.status = status.clone();
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.description {
            Some(desc) => write!(f, "[{}] {}: {}", self.get_status(), self.get_title(), desc),
            None => write!(f, "[{}] {}", self.get_status(), self.get_title()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::from("task title", Some("task description"));

        assert_eq!("task title", task.title);
        assert_eq!("task description", task.description.unwrap());
        assert_eq!(TaskStatus::UNDONE, task.status);
    }

    #[test]
    fn test_get_title() {
        let task = Task::from("task title", None);

        assert_eq!("task title", task.title);
    }

    #[test]
    fn test_set_title() {
        let mut task = Task::from("task title", None);
        task.set_title("new task title");

        assert_eq!("new task title", task.title);
    }

    #[test]
    fn test_get_description() {
        let task = Task::from("task title", Some("task description"));

        assert_eq!("task description", task.description.unwrap());
    }

    #[test]
    fn test_set_description() {
        let mut task = Task::from("task title", Some("task description"));
        task.set_description(Some("new task description"));

        assert_eq!("new task description", task.description.unwrap());
    }

    #[test]
    fn test_get_status() {
        let mut task = Task::from("task title", None);
        task.status = TaskStatus::UNDERWAY;

        assert_eq!(&TaskStatus::UNDERWAY, task.get_status());
    }

    #[test]
    fn test_set_status() {
        let mut task = Task::from("task title", None);
        task.set_status(&TaskStatus::UNDERWAY);

        assert_eq!(TaskStatus::UNDERWAY, task.status);
    }
}
