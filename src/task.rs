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

    pub fn get_title_mut(&mut self) -> &mut String {
        &mut self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_description_mut(&mut self) -> &mut String {
        &mut self.description
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
        
        assert_eq!("task title", task.get_title());
    }

    #[test]
    fn test_get_title_mut() {
        let mut task = Task::new("task title", "task description");
        
        assert_eq!("task title", task.get_title_mut());
    }

    #[test]
    fn test_get_description() {
        let task = Task::new("task title", "task description");

        assert_eq!("task description", task.get_description());
    }

    #[test]
    fn test_get_description_mut() {
        let mut task = Task::new("task title", "task description");
        
        assert_eq!("task description", task.get_description_mut());
    }

    #[test]
    fn test_is_done() {
        let task = Task::new("task title", "task description");

        assert_eq!(false, task.is_done());
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
        let task = Task::new("task title", "task description");
        assert_eq!("task title: task description", task.to_string());
    }
}