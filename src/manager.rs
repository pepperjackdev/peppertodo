use crate::task::Task;
use std::collections::HashSet;

pub struct TaskManager {
    tasks: HashSet<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: HashSet::new(),
        }
    }

    pub fn add_new_task(&mut self, task: Task) {
        self.tasks.insert(task);
    }

    pub fn get_undone_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| !task.is_done()).collect()
    }

    pub fn get_done_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.is_done()).collect()
    }

    pub fn clear_done_tasks(&mut self) {
        self.tasks.retain(|task| task.is_done());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_task() {
        let mut manager = TaskManager::new();
        let task = Task::new("title", None);

        manager.add_new_task(task);

        assert_eq!(1, manager.tasks.len());
    }

    #[test]
    fn test_get_undone_tasks() {
        let mut manager = TaskManager::new();
        let task = Task::new("title", None);
        manager.add_new_task(task);

        assert_eq!(1, manager.get_undone_tasks().len());
    }

    #[test]
    fn test_get_done_tasks() {
        let mut manager = TaskManager::new();
        let mut task = Task::new("title", None);

        task.mark_as_done();
        manager.add_new_task(task);

        assert_eq!(1, manager.get_done_tasks().len());
    }
}
