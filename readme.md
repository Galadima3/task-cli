# 🦀 [Task Tracker CLI](https://github.com/Galadima3/task-cli/) (Rust)

A simple and efficient **command-line task manager** built with Rust. Create, manage, and persist tasks using a local JSON file.

---

## ✨ Features

- Add tasks with descriptions
- List all tasks
- Edit task descriptions
- Update task status (`pending`, `in-progress`, `done`)
- Delete tasks
- Persistent JSON storage
- Automatic timestamps (created & updated)

---

## 📦 Project Structure

```
├── src
│   ├── main.rs        # CLI logic
│   ├── model.rs       # Task + status definitions
│   └── repository.rs  # File read/write logic
├── tasks.json         # Data storage (auto-created)
└── Cargo.toml
```

---

## 🛠️ Tech Stack

- **Rust**
- `serde` / `serde_json` — serialization
- `chrono` — timestamps

---

## 🚀 Getting Started

```bash
# Clone the repository
git clone https://github.com/Galadima3/task-cli.git
cd task-cli

# Build
cargo build

# Run
cargo run
```

---

## 📖 Usage

Once the CLI starts:

```
Welcome to Task Tracker CLI! Type 'help' for commands.
>
```

### Commands

| Command                   | Description              |
| ------------------------- | ------------------------ |
| `add <description>`       | Add a new task           |
| `list`                    | List all tasks           |
| `edit <id> <description>` | Edit a task              |
| `done <id>`               | Mark task as completed   |
| `in-progress <id>`        | Mark task as in progress |
| `pending <id>`            | Mark task as pending     |
| `delete <id>`             | Delete a task            |
| `exit`                    | Quit the CLI             |

---

## 🧪 Example

```
> add Learn Rust CLI
Task added successfully (ID: 1)!

> list
[1] Pending - Learn Rust CLI

> in-progress 1
Task 1 marked as in progress!

> done 1
Task 1 marked done!

> edit 1 Build a Rust CLI app
Task 1 updated successfully!

> delete 1
Task 1 deleted successfully!
```

---

## 🗂️ Data Storage

Tasks are saved to `tasks.json` (auto-created on first run):

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

| File            | Responsibility                       |
| --------------- | ------------------------------------ |
| `model.rs`      | Defines `Task` and `TodoStatus`      |
| `repository.rs` | Handles reading/writing JSON         |
| `main.rs`       | CLI loop, command parsing, and logic |

---

## ⚠️ Notes

- Task IDs are generated incrementally
- Deleting a task does **not** reuse its ID
- Invalid input is handled gracefully

---

## 🔮 Future Improvements

- [ ] Filter tasks by status (`list done`, `list pending`, etc.)
- [ ] Colored CLI output
- [ ] Search tasks
- [ ] Sort by date or status
- [ ] Export / import tasks
- [ ] TUI (Terminal UI) version

---

## 🤝 Contributing

Contributions are welcome! Fork the repo and open a pull request.

---

## 📄 License

MIT

---

*Built as a learning project to explore Rust fundamentals, CLI design, file persistence, and struct/enum modeling. Enjoy! 🦀*
