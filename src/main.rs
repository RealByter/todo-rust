mod todo_manager;
mod utils;

use todo_manager::{Todo, TodoList};

enum AppState {
    Main,
    UsingTodo(String),
}

fn create_todo(todo_list: &mut TodoList) {
    let mut name = String::new();
    let mut desc = String::new();

    println!("Please input a name");
    utils::read_non_empty_line(&mut name);

    println!("Please input a description");
    utils::read_non_empty_line(&mut desc);

    if let Err(todo_manager::Error::AlreadyExists) = todo_list.add_todo(name, desc) {
        println!("A todo with this name already exists!");
    }
}

fn print_menu(state: &AppState) {
    match state {
        AppState::Main => {
            println!("Please choose from the following options:");
            println!("1. Create a todo");
            println!("2. Print current todos");
            println!("3. Use a todo");
        }
        AppState::UsingTodo(name) => {
            println!("Using {name}:");
            println!("1. Change description");
            println!("2. Toggle completed");
            println!("3. Stop using");
        }
    }
}

fn use_todo(todo_list: &TodoList, state: &mut AppState) {
    let mut name = String::new();
    println!("Please input the name of the todo that you want to use:");
    utils::read_non_empty_line(&mut name);
    if todo_list.does_exist(&name) {
        *state = AppState::UsingTodo(name);
    } else {
        println!("Todo doesn't exist!");
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let mut state = AppState::Main;
    let mut command = String::new();

    println!("Welcome to my console todo app made in rust!");

    loop {
        print_menu(&state);
        command.clear();
        utils::read_non_empty_line(&mut command);

        if command == "quit" {
            break;
        }

        match state {
            AppState::Main => match command.trim() {
                "1" => create_todo(&mut todo_list),
                "2" => todo_list.list_todos(),
                "3" => use_todo(&todo_list, &mut state),
                _ => println!("Invalid command"),
            },
            AppState::UsingTodo(ref name) => match command.trim() {
                "1" => {}
                "2" => todo_list.toggle_todo(name),
                "3" => state = AppState::Main,
                _ => println!("Invalid command"),
            },
        }
    }
}
