mod model;
use model::*;

fn main() {
    let arg = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("---------------");

    let mut todo_list: Todo = Todo::new().expect("Initialization failed");

    if arg == "add" {
        todo_list.insert(item);
        save_list(todo_list);
    } else if arg == "complete" {
        match todo_list.complete(&item) {
            None => println!("{} is not present int the list", item),
            Some(_) => save_list(todo_list),
        }
    }
}

fn save_list(list: Todo) {
    match list.save() {
        Ok(_) => println!("list saved"),
        Err(e) => println!("An error ocurred: {}", e),
    }
}
