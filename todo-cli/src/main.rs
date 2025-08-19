use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
struct Item {
    id: i32,
    desc: String,
    done: Progress,
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
enum Progress {
    Added,
    Doing,
    Done
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

fn vec_to_file(list: Vec<Item>) {
    let count: i32 = (list.len() as i32);
    let data  = serde_json::to_string_pretty(&list).unwrap();
    fs::write("tasks.json", data).unwrap();
    println!("Task List Updated.\n{} Tasks in List", count)
}

fn save_task(task: String) {
    let mut list: Vec<Item> = file_to_vec();
    let count: i32 = (list.len() as i32) + 1;
    let task_item = Item{id: count, desc: task, done: Progress::Added};
    list.push(task_item);
    vec_to_file(list);
}

fn list_items() {
    println!("========[ RUST TODO LIST ]========\n");
    let list: Vec<Item> = file_to_vec();
    let count: i32 = (list.len() as i32);
    let mut count_done: i32 = 0;
    for item in &list {
        let icon: &str;
        match item.done {
            Progress::Added => {icon = "+"},
            Progress::Doing => {icon = "-"},
            Progress::Done => {count_done+= 1; icon = "✔"},
        }
        println!("{}. {}: {}", item.id, item.desc, icon)
    }
    println!("\n{}/{} Tasks Complete \n========[ RUST TODO LIST ]========", count_done, count);
}

fn set_complete(task: String, inProgress: bool) {
    let mut index: i32 = 0;
    match task.parse::<i32>() {
        Ok(num) => {index = num;}
        Err(e) => {println!("Error! Please enter a valid index number: {}", e); return}
    }
    let mut list: Vec<Item> = file_to_vec();
    if let Some(v) = list.get_mut(index as usize) {
        match inProgress {
        false => {
            v.done = Progress::Done;
            println!("Task '{}' has been Completed.", v.desc);
        }
        true => {
            v.done = Progress::Doing;
            println!("Task '{}' is in Progress.", v.desc);
        }
        }
        vec_to_file(list);
        return;
    }

    println!("Error! Please enter a valid index number.");
    return;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let option: &String = &args[1];

    match option.to_lowercase().as_str() {
        "add" => { 
            let item: String = args[2].clone(); 
            save_task(item) 
        }
        "list" => { list_items() }
        "done" => { let task: String = args[2].clone(); set_complete(task, false) }
        "doing" => { let task: String = args[2].clone(); set_complete(task, true) }
        _ => { eprint!("Invalid option. Use 'add' or 'list'") }
    }
}