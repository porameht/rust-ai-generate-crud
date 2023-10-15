use std::error::Error;
use async_trait::async_trait; // Import the async_trait macro.
use crate::domain::todo::Todo;
#[async_trait]
pub trait TodoRepository {
    async fn create(&self, todo: Todo) -> Result<Todo, Box<dyn Error>>;
    async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn Error>>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Todo>, Box<dyn Error>>;
    async fn update(&self, id: &str, todo: Todo) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
