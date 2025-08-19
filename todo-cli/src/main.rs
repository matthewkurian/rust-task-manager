use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
struct Item {
    id: i32,
    desc: String,
    done: bool
}

fn file_to_vec() -> Vec<Item> {
    if !Path::new("tasks.json").exists() {
        return Vec::new();
    }

    let data = fs::read_to_string("tasks.json").unwrap();
    let list: Vec<Item> = serde_json::from_str(&data)
    .expect("JSON was not well-formatted");
    list
}

fn vec_to_file(list: Vec<Item>, count: i32) {
    let data  = serde_json::to_string_pretty(&list).unwrap();
    fs::write("tasks.json", data).unwrap();
    println!("Task List Updated.\n{} Tasks in List", count)
}

fn save_task(task: String) {
    let mut list: Vec<Item> = file_to_vec();
    let count: i32 = (list.len() as i32) + 1;
    let task_item = Item{id: count, desc: task, done: false};
    list.push(task_item);
    vec_to_file(list, count);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let option: &String = &args[1];
    let item: String = args[2].clone();
    
    match option.to_lowercase().as_str() {
        "add" => { save_task(item) }
        "list" => { println!("ok") }
        _ => { eprint!("Invalid option. Use 'add' or 'list'") }
    }
}