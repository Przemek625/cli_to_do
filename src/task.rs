use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub is_completed: bool,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.title, self.is_completed)
    }
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            is_completed: false,
            tags: None,
            created_at: Some(Utc::now()),
            completed_at: None,
        }
    }
    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = Some(tags);
    }

    pub fn set_done(&mut self) {
        self.is_completed = true;
        self.completed_at = Some(Utc::now());
    }
}
