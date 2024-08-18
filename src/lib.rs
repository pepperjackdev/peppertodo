
pub struct Task {
    title: String,
    description: String,
    is_done: bool,
}

impl Task {
    /// Creates a new task
    pub fn new(title: &str, description: &str) -> Task {
        let title = title.to_string();
        let description = description.to_string();

        Task {
            title,
            description,
            is_done: false,
        }
    }

    // Returns the title of the task
    pub fn title(&self) -> &str {
        &self.title
    }

    // Returns the description of the task
    pub fn description(&self) -> &str {
        &self.description
    }

    // Gets the status
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    // Marks a task as done
    pub fn mark_as_done(&mut self) {
        self.is_done = true;
    }

    // Marks a task as undone
    pub fn mark_as_undone(&mut self) {
        self.is_done = false;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_task() {
        let title = "Do something!";
        let description = "Don't know why, but do that";
        let task = Task::new(title, description);

        assert_eq!(title, task.title);
        assert_eq!(description, task.description);
        assert!(!task.is_done);
    }

    #[test]
    fn should_return_title() {
        let title = "Task name";
        let task = Task {
            title: title.to_string(),
            description: "Task description".to_string(),
            is_done: false,
        };

        assert_eq!(title, task.title());
    }

    #[test]
    fn should_return_description() {
        let description = "By doing nothing, you're doing this task. So you're doing something.";
        let task = Task {
            title: "Do nothing!".to_string(),
            description: description.to_string(),
            is_done: false,
        };

        assert_eq!(description, task.description());
    }

    #[test]
    fn should_return_done_status() {
        let task = Task {
            title: "Title".to_string(),
            description: "Description".to_string(),
            is_done: false,
        };

        assert!(!task.is_done());
    }

    #[test]
    fn should_mark_task_as_done() {
        let mut task = Task {
            title: "Remember to breathe every 1 or 2 seconds".to_string(),
            description: "By accomplishing this task, you're increasing your chances of survival".to_string(),
            is_done: false,
        };

        task.mark_as_done();

        assert!(task.is_done)
    }

    #[test]
    fn should_mark_task_as_undone() {
        let mut task = Task {
            title: "A task title".to_string(),
            description: "A task description".to_string(),
            is_done: true,
        };

        task.mark_as_undone();

        assert!(!task.is_done());
    }
}