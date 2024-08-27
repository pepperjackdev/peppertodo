use std::{error::Error, io::ErrorKind};

use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_new_task(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        let already_existing_task = self.get_task(task.get_title());
        if already_existing_task.is_some() {
            Err(Box::new(std::io::Error::new(
                ErrorKind::AlreadyExists,
                "Task already exists",
            )))
        } else {
            self.tasks.push(task);
            Ok(())
        }
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_task(&self, title: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.get_title() == title)
    }

    pub fn get_task_mut(&mut self, title: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.get_title() == title)
    }

    pub fn delete_task(&mut self, title: &str) -> Result<(), Box<dyn Error>> {
        let to_remove = self.get_task(title);
        match to_remove {
            Some(_) => Ok(self.tasks.retain(|t| t.get_title() != title)),
            None => Err(Box::new(std::io::Error::new(
                ErrorKind::NotFound,
                "Task not found",
            ))),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_task() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);

        let _ = manager.add_new_task(task);

        assert!(!manager.tasks.is_empty());
    }

    #[test]
    fn test_add_new_task_with_taken_title() {
        let mut manager = TaskManager::new();
        let first_task = Task::from("task title", None);
        let second_task = Task::from("task title", Some("maybe a description"));

        let _ = manager.add_new_task(first_task);
        let result = manager.add_new_task(second_task);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_all_tasks() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);
        let _ = manager.add_new_task(task);

        let tasks = manager.get_all_tasks();

        assert_eq!(&manager.tasks, tasks);
    }

    #[test]
    fn test_get_task() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);
        let _ = manager.add_new_task(task);

        let task: Option<&Task> = manager.get_task("task title");

        assert!(task.is_some());
    }

    #[test]
    fn test_get_task_mut() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);
        let _ = manager.add_new_task(task);

        let task: Option<&mut Task> = manager.get_task_mut("task title");

        assert!(task.is_some());
    }

    #[test]
    fn test_delete_task() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);
        let _ = manager.add_new_task(task);

        let result = manager.delete_task("task title");

        assert!(result.is_ok());
    }
}
