
# 📝 Rust Task Manager CLI

A simple **command-line task manager** written in [Rust](https://www.rust-lang.org/).  
This project is designed as a learning journey: starting from a basic CLI app and expanding into a more feature-rich tool over time.

---

## ✨ Features

- Add tasks directly from the command line
- List all tasks with their completion status
- Mark tasks as done
- Data persisted to a local JSON file
- Lightweight and cross-platform

Planned future features:
- Categories/tags
- Terminal UI (TUI)
- Reminders and notifications
- Web API for syncing tasks

---

## 🚀 Getting Started

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install) (uses `cargo` for builds)

Verify installation:
```bash
rustc --version
cargo --version
````

### Clone the repository

```bash
git clone https://github.com/matthewkurian/rust-task-manager.git
cd todo-cli
```

### Build the project

```bash
cargo build --release
```

The compiled binary will be available at:

```
target/release/todo-cli
```

### Install locally (optional)

```bash
cargo install --path .
```

This lets you run `todo-cli` from anywhere on your system.

---

## 📖 Usage

```bash
todo-cli add "Buy milk"
todo-cli add "Learn Rust"
todo-cli list
todo-cli done 1
```

Example output:

```
1. Buy milk: +
2. Learn Rust: +

0/2 Tasks Complete 
```

After marking task 1 as done:

```
1. Buy milk: ✔
2. Learn Rust: +

1/2 Tasks Complete 
```

---

## 🛠 Project Structure

```
src/
  main.rs       # CLI entry point
  models.rs     # Task struct and related types (planned)
  storage.rs    # Load/save logic (planned)
tasks.json      # JSON file used to persist tasks
```

---

## 🤝 Contributing

Pull requests and suggestions are welcome!
This project is intended as a Rust learning project, so clear explanations and teaching-focused contributions are appreciated :)

---

## 📜 License

This project is licensed under the **MIT License** – see the [LICENSE](LICENSE) file for details.


---
