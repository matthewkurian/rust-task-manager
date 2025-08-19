use serde::{Serialize, Deserialize};
#[derive(serde::Serialize)]

struct Item {
    id: i32,
    desc: String,
    done: bool
}

fn save_task(task: Item) {
    let json = serde_json::to_string_pretty(&task).unwrap();
    println!("New Task Added \n{}", json)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let option: &String = &args[1];
    let item = Item{id: 1, desc: args[2].clone(), done: false};

    if option.to_lowercase() == "add" {
        save_task(item);
    } else if option.to_lowercase() == "list" {
        println!("List Triggered")
    } else {
        eprintln!("Invalid option. Use 'add' or 'list'")
    }

}