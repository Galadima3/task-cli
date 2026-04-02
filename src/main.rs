use std::io::{self, Write};

use chrono::Utc;

use crate::{
    model::{Task, TodoStatus},
    repository::{load_tasks, save_tasks},
};

mod model;
mod repository;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("Error: {}", e)
    }
}
fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "tasks.json";
    let mut tasks = load_tasks(filename);
    println!("Welcome to Task Tracker CLI! Type 'help' for commands.");

    loop {
        println!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => break,
            "help" => {
                println!("Commands:");
                println!(" add <description>       - Add a new task");
                println!(" list                    - List all tasks");
                println!(" list done               - List completed tasks");
                println!(" list in-progress        - List tasks in progress");
                println!(" list pending            - List pending tasks");
                println!(" done <id>               - Mark task as done");
                println!(" in-progress <id>        - Mark task as in-progress");
                println!(" pending <id>            - Mark task as pending");
                println!(" delete <id>             - Delete a task");
                println!(" exit                    - Quit CLI");
            }
            "add" => {
                if args.is_empty() {
                    println!("Usage: add <description>");
                    continue;
                }
                let description = args.join(" ");
                let id = tasks.last().map_or(1, |t| t.id + 1);
                let now = Utc::now();
                tasks.push(Task {
                    id,
                    description,
                    status: TodoStatus::InProgress,
                    created_at: now,
                    updated_at: now,
                });
                save_tasks(filename, &tasks);
                println!("New Task added!");
            }
            "list" => {
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    for task in &tasks {
                        println!("[{}] {} - {}", task.id, task.status, task.description);
                    }
                }
            }
            "done" | "in-progress" => {
                if args.len() < 1 {
                    println!("Usage: {} <id>", command);
                    continue;
                }
                let id: u32 = match args[0].parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };
                if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                    task.status = match command {
                        "done" => TodoStatus::Completed,
                        "in-progress" => TodoStatus::InProgress,
                        _ => task.status.clone(),
                    };
                    task.updated_at = Utc::now();
                    save_tasks(filename, &tasks);
                    println!("Task {} marked {}!", id, command);
                } else {
                    println!("Task ID not found");
                }
            }
            _ => println!("Unknown command. Type 'help' for commands."),
        }
    }
    Ok(())
}
