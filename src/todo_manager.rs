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
            return Err(Error::AlreadyExists);
        }

        self.todos.insert(name, Todo::new(description));
        Ok(())
    }

    pub fn list_todos(&self) {
        for (name, todo) in &self.todos {
            println!("{:?}", todo);
            println!(
                "[{}] Name: {name}, Description: {}",
                if todo.get_completed() { "X" } else { " " },
                todo.get_description()
            )
        }
    }
}
