#![allow(unused_parens)]

use core::task;

use crate::models::Item;
use crate::models::Progress;
use crate::storage::file_to_vec;
use crate::storage::vec_to_file;

mod models;
mod storage;


fn save_task(task: String, task_list: &mut Vec<Item>) {
    let count: i32 = (task_list.len() as i32) + 1;
    let task_item = Item{id: count, desc: task, done: Progress::Added};
    task_list.push(task_item);
}

fn list_items(task_list: &Vec<Item>) {
    println!("========[ RUST TODO LIST ]========\n");
    let count: i32 = (task_list.len() as i32);
    let mut count_done: i32 = 0;
    for item in task_list {
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

fn set_complete(task: String, in_progress: bool, task_list: &mut Vec<Item>) {
    let index: i32;
    match task.parse::<i32>() {
        Ok(num) => {index = num-1;}
        Err(e) => {println!("Error! Please enter a valid index number: {}", e); return}
    }
    if let Some(v) = task_list.get_mut(index as usize) {
        match in_progress {
        false => {
            v.done = Progress::Done;
            println!("Task '{}' has been Completed.", v.desc);
        }
        true => {
            v.done = Progress::Doing;
            println!("Task '{}' is in Progress.", v.desc);
        }
        }
        return;
    }

    println!("Error! Please enter a valid index number.");
    return;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let option: &String = &args[1];
    let mut task_list: Vec<Item>;

    // Get Task List
    match file_to_vec() {
        Ok(v) => {task_list = v},
        Err(e) => {println!("⚠️ Could not load tasks: {}", e); return}
    }

    match option.to_lowercase().as_str() {
        "add" => { 
            let item: String = args[2].clone(); 
            save_task(item, &mut task_list) 
        }
        "list" => { list_items(&task_list) }
        "done" => { let task: String = args[2].clone(); set_complete(task, false, &mut task_list) }
        "doing" => { let task: String = args[2].clone(); set_complete(task, true, &mut task_list) }
        _ => { eprint!("Invalid option. Use 'add' or 'list'") }
    }

    // Save Vec to file
    match vec_to_file(task_list) {
        Ok(()) => {},
        Err(e) => {println!("⚠️ Could not save tasks: {}", e)}
    }

}