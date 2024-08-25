use std::error::Error;

use rusqlite::{params, Connection};

use crate::task::Task;

pub struct TaskManager {
    connection: Connection,
}

impl TaskManager {
    pub fn new() -> Result<TaskManager, Box<dyn Error>> {
        let connection_result = Connection::open("appdata.db");
        match connection_result {
            Ok(connection) => {
                // initializing the tasks table if not present
                let _ = connection.execute("CREATE TABLE IF NOT EXISTS tasks ('title' text, 'description' text, )", params![]);
                Ok(TaskManager {
                    connection
                })
            },
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn add_new_task(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        self.connection.execute()
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        todo!()
    }

    pub fn get_task(&self, title: &str) -> Result<&Task, Box<dyn Error>> {
        todo!()
    }

    pub fn get_task_mut(&mut self, title: &str) -> Result<&mut Task, Box<dyn Error>> {
        todo!()
    }

    pub fn delete_task(&mut self, title: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::TaskManager;


    #[test]
    fn test_new_taskmanager() {
        let manager_result = TaskManager::new();
        assert!(manager_result.is_ok())
    }

    #[test]
    fn test_add_new_task() {
        todo!()
    }

    #[test]
    fn test_add_two_tasks_with_same_title() {
        todo!()
    }

    #[test]
    fn test_get_all_tasks() {
        todo!()
    }

    #[test]
    fn test_get_task_by_title() {
        todo!()
    }

    #[test]
    fn test_get_mut_task_by_title() {
        todo!()
    }

    #[test]
    fn test_delete_task() {
        todo!()
    }
}
