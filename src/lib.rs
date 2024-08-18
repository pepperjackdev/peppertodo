
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