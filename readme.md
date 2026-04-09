````md
# 🦀 Task Tracker CLI (Rust)

A simple and efficient **command-line task manager** built with Rust.  
This CLI app allows you to create, manage, and persist tasks using a JSON file.

---

## ✨ Features

- ✅ Add tasks with descriptions  
- 📋 List all tasks  
- ✏️ Edit task descriptions  
- 🔄 Update task status (`pending`, `in-progress`, `done`)  
- ❌ Delete tasks  
- 💾 Persistent storage using JSON  
- ⏱️ Automatic timestamps (created & updated)

---

## 📦 Project Structure

```
.
├── src
│   ├── main.rs        # CLI logic
│   ├── model.rs       # Task + status definitions
│   └── repository.rs  # File read/write logic
├── tasks.json         # Data storage (auto-created)
└── Cargo.toml
```

---

## 🛠️ Tech Stack

- Rust
- serde / serde_json (for serialization)
- chrono (for timestamps)

---

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/your-username/task-tracker-cli.git
cd task-tracker-cli
```

### 2. Install dependencies

```bash
cargo build
```

### 3. Run the app

```bash
cargo run
```

---

## 📖 Usage

Once the CLI starts:

```
Welcome to Task Tracker CLI! Type 'help' for commands.
>
```

### Available Commands

| Command | Description |
|--------|------------|
| `add <description>` | Add a new task |
| `list` | List all tasks |
| `edit <id> <description>` | Edit a task |
| `done <id>` | Mark task as completed |
| `in-progress <id>` | Mark task as in progress |
| `pending <id>` | Mark task as pending |
| `delete <id>` | Delete a task |
| `exit` | Quit the CLI |

---

## 🧪 Example

```bash
> add Learn Rust CLI
Task added successfully (ID: 1)!

> list
[1] In Progress - Learn Rust CLI

> done 1
Task 1 marked done!

> list
[1] Completed - Learn Rust CLI

> edit 1 Build a Rust CLI app
Task 1 updated successfully!

> delete 1
Task 1 deleted successfully!
```

---

## 🗂️ Data Storage

Tasks are stored in a local file:

```
tasks.json
```

Example structure:

```json
[
  {
    "id": 1,
    "description": "Learn Rust CLI",
    "status": "InProgress",
    "created_at": "2026-04-09T12:00:00Z",
    "updated_at": "2026-04-09T12:10:00Z"
  }
]
```

---

## 🧠 How It Works

- **`model.rs`** → Defines `Task` and `TodoStatus`
- **`repository.rs`** → Handles reading/writing JSON
- **`main.rs`** → CLI loop, command parsing, and logic

---

## ⚠️ Notes

- Task IDs are generated incrementally
- Deleting tasks does **not** reuse IDs
- Invalid input is handled gracefully

---

## 🔮 Future Improvements

- [ ] Filter tasks (`list done`, `list pending`, etc.)
- [ ] Colored CLI output
- [ ] Search tasks
- [ ] Sort by date/status
- [ ] Export/import tasks
- [ ] TUI (Terminal UI) version

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork and improve the project.

---

## 📄 License

MIT License

---

## 🙌 Acknowledgements

Built as a learning project to practice:
- Rust fundamentals
- CLI design
- File persistence
- Struct & enum modeling

---

Enjoy building with Rust! 🦀
````
