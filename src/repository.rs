use std::fs;

use crate::model::Task;

// Read tasks
pub fn load_tasks(filename: &str) -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(filename) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}
// Create Tasks
pub fn save_tasks(filename: &str, tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(filename, data).expect("Unable to write file");
}

