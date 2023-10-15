use mongodb::{Client, Collection};
use mongodb::bson::{doc, to_document, from_document};
use std::error::Error;
use async_trait::async_trait; // Import the async_trait macro.
use async_std::stream::StreamExt;

use crate::domain::todo::Todo;
use crate::domain::repository::TodoRepository;

#[derive(Clone)]
pub struct MongoTodoRepository {
    client: Client,
}

impl MongoTodoRepository {
    pub async fn new(mongo_uri: &str) -> Self {
        let client = Client::with_uri_str(mongo_uri).await.expect("Failed to connect to MongoDB");
        MongoTodoRepository { client }
    }
}

#[async_trait]
impl TodoRepository for MongoTodoRepository {
    async fn create(&self, todo: Todo) -> Result<Todo, Box<dyn Error>> {
        let db = self.client.database("db");
        let collection = db.collection("todos");

        let todo_doc = to_document(&todo)?;

        collection.insert_one(todo_doc, None).await?;
        Ok(todo)
    }

    async fn find_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let db = self.client.database("db");
        let collection = db.collection("todos");

        let mut cursor = collection.find(None, None).await?;
        let mut todos = vec![];

        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    let todo: Todo = from_document(doc)?;
                    todos.push(todo);
                }
                Err(e) => {
                    // Handle the error as needed
                    eprintln!("Error while iterating the cursor: {}", e);
                }
            }
        }
        

        Ok(todos)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Todo>, Box<dyn Error>> {
        let db = self.client.database("db");
        let collection = db.collection("todos");
        // log the id to see if it is working
        println!("{} eiei", id);

        let filter = doc! { "id": id };
        if let Some(doc) = collection.find_one(filter, None).await? {
            let todo: Todo = from_document(doc)?;
            Ok(Some(todo))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, id: &str, todo: Todo) -> Result<(), Box<dyn Error>> {
        let db = self.client.database("db");
        let collection: Collection<Todo> = db.collection("todos");

        let filter = doc! { "id": id };
        let update = doc! { "$set": to_document(&todo)? };

        collection.update_one(filter, update, None).await?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let db = self.client.database("db");
        let collection: Collection<Todo> = db.collection("todos");

        let filter = doc! { "id": id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}