use std::fmt::{self};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            TodoStatus::Pending => "Pending",
            TodoStatus::InProgress => "In Progress",
            TodoStatus::Completed => "Completed",
        };
        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TodoStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

