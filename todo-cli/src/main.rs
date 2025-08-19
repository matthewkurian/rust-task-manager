use serde::{Serialize, Deserialize};
use std::fs;
use std::io::prelude::*;
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

fn save_task(task: String) {
    let mut list: Vec<Item> = file_to_vec();
    let task_item = Item{id: (list.len() as i32) + 1, desc: task, done: false};
    list.push(task_item);
    
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let option: &String = &args[1];
    let item: String = args[2].clone();

    if option.to_lowercase() == "add" {
        save_task(item);
    } else if option.to_lowercase() == "list" {
        println!("List Triggered")
    } else {
        eprintln!("Invalid option. Use 'add' or 'list'")
    }
}