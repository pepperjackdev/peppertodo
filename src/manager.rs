use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new()
        }
    }

    pub fn add_new_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use crate::task::Task;

    use super::TaskManager;


    #[test]
    fn test_add_new_task() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);

        manager.add_new_task(task);

        assert!(!manager.tasks.is_empty());
    }

    #[test]
    fn test_get_all_tasks() {
        let mut manager = TaskManager::new();
        let task = Task::from("task title", None);
        manager.add_new_task(task);

        let all_tasks = manager.get_all_tasks();

        assert_eq!(manager.tasks, *all_tasks);
    }
}