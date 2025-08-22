#![allow(unused_parens)]

use crate::models::Item;
use crate::models::Progress;
use crate::storage::file_to_vec;
use crate::storage::vec_to_file;
use clap::{Parser, Subcommand};

mod models;
mod storage;

#[derive(Parser)]
#[command(name = "todo-cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// List items in the task list
    List {},

    /// Add item to task list
    Add {
        item: String
    },

    Done {
        id: i32,

        #[arg(short, long)]
        doing: bool
    }
}


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

fn set_complete(task: i32, in_progress: bool, task_list: &mut Vec<Item>) {
    let index: i32 = task - 1;
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
    let mut task_list: Vec<Item>;

    // Get Task List
    match file_to_vec() {
        Ok(v) => {task_list = v},
        Err(e) => {println!("⚠️ Could not load tasks: {}", e); return}
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::List {} => {
            list_items(&task_list)
        }
        Commands::Add { item } => {
            save_task(item, &mut task_list);
        }
        Commands::Done { id, doing } => {
            set_complete(id, doing, &mut task_list);
        }
    }

    // Save Vec to file
    match vec_to_file(task_list) {
        Ok(()) => {},
        Err(e) => {println!("⚠️ Could not save tasks: {}", e)}
    }

}