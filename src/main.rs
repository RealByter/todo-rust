mod task;

use std::collections::HashMap;
use std::io;
use task::Todo;

fn read_non_empty_line(buf: &mut String) {
    loop {
        io::stdin().read_line(buf).expect("Failed to read line");

        *buf = buf.trim().to_string();

        if buf.is_empty() {
            println!("Can't enter an empty line. Please try again");
        } else {
            break;
        }
    }
}

fn main() {
    let mut todos: HashMap<String, Todo> = HashMap::new();

    loop {
        let mut name = String::new();
        let mut desc = String::new();

        println!("Please input a name");

        read_non_empty_line(&mut name);

        if name == "quit" {
            break;
        } else if todos.contains_key(&name) {
            println!("Can't create 2 todos with the same name");
            continue;
        }

        println!("Please input a description");

        read_non_empty_line(&mut desc);

        let todo = Todo::new(desc);
        todos.insert(name, todo);
        // println!("{:?}", todos);
    }

    // let mut first = Todo::new(desc);
    // first.complete();
    // let desc = first.get_description();
    // let complete = first.get_completed();
    // println!(
    //     "You {} todo with description: {}",
    //     if complete {
    //         "completed"
    //     } else {
    //         "didn't complete"
    //     },
    //     desc.trim()
    // );
}
