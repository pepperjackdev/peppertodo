use std::fmt::Display;

pub struct Task {
    title: String,
    description: String,
    is_done: bool,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Task {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            is_done: false,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn mark_as_done(&mut self) {
        self.is_done = true;
    }

    pub fn mark_as_undone(&mut self) {
        self.is_done = false;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_title(), self.get_description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new("task title", "task description");

        assert_eq!("task title", task.title);
        assert_eq!("task description", task.description);
        assert!(!task.is_done);
    }

    #[test]
    fn test_get_title() {
        let task = Task::new("task title", "task description");
        
        assert_eq!("task title", task.title);
    }

    #[test]
    fn test_set_title() {
        let mut task = Task::new("task title", "task description");
        task.set_title("new task title");
        
        assert_eq!("new task title", task.title);
    }

    #[test]
    fn test_get_description() {
        let task = Task::new("task title", "task description");

        assert_eq!("task description", task.description);
    }

    #[test]
    fn test_set_description() {
        let mut task = Task::new("task title", "task description");
        task.set_description("new task description");
        
        assert_eq!("new task description", task.description);
    }

    #[test]
    fn test_is_done() {
        let task = Task::new("task title", "task description");

        assert!(!task.is_done());
    }

    #[test]
    fn test_mark_as_done() {
        let mut task = Task::new("task title", "task description");
        
        task.mark_as_done();

        assert!(task.is_done);
    }

    #[test]
    fn test_mark_as_undone() {
        let mut task = Task::new("task title", "task description");
        
        task.is_done = true;
        task.mark_as_undone();

        assert!(!task.is_done);
    }

    #[test]
    fn test_to_string() {
        let task = Task::new("Task title", "task description");
        assert_eq!("Task title: task description", task.to_string());
    }
}