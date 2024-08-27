use std::error::Error;

use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_new_task(&mut self, task: Task) -> Result<(), Box<&'static str>> {
        if let Some(_) = self
            .tasks
            .iter()
            .find(|t| t.get_title() == task.get_title())
        {
            return Err(Box::new("Title already used in another task"));
        }
        self.tasks.push(task);
        Ok(())
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_task(&self, title: &str) -> Result<&Task, Box<&'static str>> {
        let task_option = self.tasks.iter().find(|task| task.get_title() == title);
        match task_option {
            Some(task) => Ok(task),
            None => Err(Box::new("No task found with that title")),
        }
    }

    pub fn get_task_mut(&mut self, title: &str) -> Result<&mut Task, Box<&'static str>> {
        let task_option = self.tasks.iter_mut().find(|task| task.get_title() == title);
        match task_option {
            Some(task) => Ok(task),
            None => Err(Box::new("No task found with that title")),
        }
    }

    pub fn delete_task(&mut self, title: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_taskmanager() {

    use super::TaskManager;

    #[test]
    fn test_add_new_task() {
        
    }

    #[test]
    fn test_add_two_tasks_with_same_title() {
        let mut manager = TaskManager::new();
        let task_a = Task::from("task title", None);
        let task_b = Task::from("task title", None);

        let _ = manager.add_new_task(task_a);
        let result = manager.add_new_task(task_b);

        assert_eq!(Err(Box::new("Title already used in another task")), result);
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