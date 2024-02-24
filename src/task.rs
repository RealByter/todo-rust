#[derive(Debug)]
pub struct Todo {
    completed: bool,
    description: String
}

impl Todo {
    pub fn new(desc: String) -> Self {
        Todo { completed: false, description: desc }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn get_completed(&self) -> bool {
      self.completed
    }

    pub fn get_description(&self) -> &str {
      &self.description
    }
}
