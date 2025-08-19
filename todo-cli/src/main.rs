
struct Item {
    id: i32,
    desc: String,
    done: bool
}

fn main() {
    let task1 = Item{id: 1, desc: "Pick up food".to_string(), done: false};
    println!("{}", task1.desc);
}