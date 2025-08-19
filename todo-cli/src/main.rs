
struct Item {
    id: i32,
    desc: String,
    done: bool
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let option: &String = &args[1];
    let item: &String = &args[2];

    if option.to_lowercase() == "add" {
        println!("Add Triggered: {}", item)
    } else if option.to_lowercase() == "list" {
        println!("List Triggered")
    } else {
        eprintln!("Invalid option. Use 'add' or 'list'")
    }

}