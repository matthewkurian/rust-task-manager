use std::fs;
use std::path::Path;
use anyhow::Result;

use crate::models::Item;

pub fn file_to_vec() -> Result<Vec<Item>> {
    if !Path::new("tasks.json").exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string("tasks.json")?;
    let list: Vec<Item> = serde_json::from_str(&data)?;
    Ok(list)
}

pub fn vec_to_file(list: Vec<Item>) -> Result<()> {
    let data  = serde_json::to_string_pretty(&list)?;
    fs::write("tasks.json", data)?;
    Ok(())
}