use std::fmt::Display;

#[derive(PartialEq, Eq, Hash)]
pub struct Task {
    title: String,
    description: Option<String>,
    is_done: bool,
}

impl Task {
    pub fn new(title: &str, description: Option<&str>) -> Task {
        let description = match description {
            Some(description) => Some(description.to_string()),
            None => None,
        };

        Task {
            title: title.to_string(),
            description: description,
            is_done: false,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn get_description(&self) -> Option<&str> {
        match &self.description {
            Some(desc) => Some(&desc),
            None => None,
        }
    }

    pub fn set_description(&mut self, description: Option<&str>) {
        if let Some(desc) = description {
            self.description = Some(desc.to_string());
        }
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
        match &self.description {
            Some(desc) => write!(f, "{}: {}", self.get_title(), desc),
            None => write!(f, "{}", self.get_title()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new("task title", Some("task description"));

        assert_eq!("task title", task.title);
        assert_eq!("task description", task.description.unwrap());
        assert!(!task.is_done);
    }

    #[test]
    fn test_get_title() {
        let task = Task::new("task title", None);

        assert_eq!("task title", task.title);
    }

    #[test]
    fn test_set_title() {
        let mut task = Task::new("task title", None);
        task.set_title("new task title");

        assert_eq!("new task title", task.title);
    }

    #[test]
    fn test_get_description() {
        let task = Task::new("task title", Some("task description"));

        assert_eq!("task description", task.description.unwrap());
    }

    #[test]
    fn test_set_description() {
        let mut task = Task::new("task title", Some("task description"));
        task.set_description(Some("new task description"));

        assert_eq!("new task description", task.description.unwrap());
    }

    #[test]
    fn test_is_done() {
        let task = Task::new("task title", None);

        assert!(!task.is_done());
    }

    #[test]
    fn test_mark_as_done() {
        let mut task = Task::new("task title", None);

        task.mark_as_done();

        assert!(task.is_done);
    }

    #[test]
    fn test_mark_as_undone() {
        let mut task = Task::new("task title", None);

        task.is_done = true;
        task.mark_as_undone();

        assert!(!task.is_done);
    }

    #[test]
    fn test_to_string_with_no_description() {
        let task = Task::new("Task title", None);

        assert_eq!("Task title", task.to_string());
    }

    #[test]
    fn test_to_string_with_description() {
        let task = Task::new("Task title", Some("task description"));

        assert_eq!("Task title: task description", task.to_string());
    }
}
