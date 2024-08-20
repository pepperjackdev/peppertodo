use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: vec![],
        }
    }

    pub fn add_new_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_new_task() {
        let mut manager = TaskManager::new();
        let task = Task::new("title", "description");

        manager.add_new_task(task);

        assert_eq!(1, manager.tasks.len());
    }
}