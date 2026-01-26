use serde::{Deserialize, Serialize};
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub is_completed: bool,
}
