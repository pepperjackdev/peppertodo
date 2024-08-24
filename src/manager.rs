use crate::task::Task;

pub struct TaskManager {
    
}

impl TaskManager {
    pub fn new() -> TaskManager {
        todo!()
    }

    pub fn add_new_task(&mut self, task: Task) -> Result<(), Box<&'static str>> {
        todo!()
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        todo!()
    }

    pub fn get_task(&self, title: &str) -> Result<&Task, Box<&'static str>> {
        todo!()
    }

    pub fn get_task_mut(&mut self, title: &str) -> Result<&mut Task, Box<&'static str>> {
        todo!()
    }

    pub fn delete_task(&mut self, title: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

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
