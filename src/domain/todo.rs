use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub completed: bool,
}