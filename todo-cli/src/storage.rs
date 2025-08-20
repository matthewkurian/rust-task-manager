use std::fs;
use std::path::Path;

use crate::models::Item;

pub fn file_to_vec() -> Vec<Item> {
    if !Path::new("tasks.json").exists() {
        return Vec::new();
    }

    let data = fs::read_to_string("tasks.json").unwrap();
    let list: Vec<Item> = serde_json::from_str(&data)
    .expect("JSON was not well-formatted");
    list
}

pub fn vec_to_file(list: Vec<Item>) {
    let data  = serde_json::to_string_pretty(&list).unwrap();
    fs::write("tasks.json", data).unwrap();
}