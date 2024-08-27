use std::error::Error;

use crate::task::Task;

pub struct TaskManager {
    
}

impl TaskManager {
    pub fn new() -> Result<TaskManager, Box<dyn Error>> {
        todo!()
    }

    pub fn add_new_task(&self, task: Task) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        todo!()
    }

    pub fn get_task(&self, title: &str) -> Result<Task, Box<dyn Error>> {
        todo!()
    }

    pub fn delete_task(&mut self, title: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_taskmanager() {

    }

    #[test]
    fn test_add_new_task() {
        
    }

    #[test]
    fn test_add_two_tasks_with_same_title() {

    }

    #[test]
    fn test_get_all_tasks() {

    }

    #[test]
    fn test_get_task_by_title() {

    }

    #[test]
    fn test_delete_task() {

    }
}
