#[derive(Debug)]
pub struct Todo {
    completed: bool,
    description: String,
}

impl Todo {
    pub fn new(desc: String) -> Self {
        Todo {
            completed: false,
            description: desc,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
