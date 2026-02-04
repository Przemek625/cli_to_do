use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub is_completed: bool,
}

impl Task {
    fn set_done(&mut self) {
        self.is_completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.id, self.title, self.is_completed)
    }
}
