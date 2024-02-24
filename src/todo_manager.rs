mod todo;

use std::collections::HashMap;
pub use todo::Todo;

pub enum Error {
    AlreadyExists,
}

pub struct TodoList {
    todos: HashMap<String, Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: HashMap::new(),
        }
    }

    pub fn add_todo(&mut self, name: String, description: String) -> Result<(), Error> {
        if self.todos.contains_key(&name) {
            println!("Can't create 2 todos with the same name");
            return Err(Error::AlreadyExists);
        }

        self.todos.insert(name, Todo::new(description));
        Ok(())
    }
}
