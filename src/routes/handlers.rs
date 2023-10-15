// handlers.rs
use crate::domain::repository::TodoRepository;
use crate::infrastructure::mongo::MongoTodoRepository;
use crate::domain::todo::Todo;
use warp::Reply;

pub async fn create_todo_handler(todo: Todo, repository: MongoTodoRepository) -> Result<impl Reply, warp::Rejection> {
    let created_todo = repository.create(todo).await.unwrap();
    Ok(warp::reply::json(&created_todo))
}

pub async fn get_todos_handler(repository: MongoTodoRepository) -> Result<impl Reply, warp::Rejection> {
    let todos = repository.find_all().await.unwrap();
    Ok(warp::reply::json(&todos))
}

pub async fn get_todo_handler(id: String, repository: MongoTodoRepository) -> Result<impl Reply, warp::Rejection> {
    // log the id to see if it is working
    println!("{}", id);
    let todo = repository.find_by_id(&id).await.unwrap();
    
    println!("{:?} todo", todo);
    match todo {
        Some(todo) => Ok(warp::reply::json(&todo)),
        None => {
            let response = warp::reply::json(&"Resource not found"); // You can change the message as needed.
            Ok(response)
        }
        
    }
}

pub async fn update_todo_handler(id: String, todo: Todo, repository: MongoTodoRepository) -> Result<impl Reply, warp::Rejection> {
    let updated_todo = repository.update(&id, todo).await.unwrap();
    Ok(warp::reply::json(&updated_todo))
}

pub async fn delete_todo_handler(id: String, repository: MongoTodoRepository) -> Result<impl Reply, warp::Rejection> {
    repository.delete(&id).await.unwrap();
    Ok(warp::reply::with_status("", warp::http::StatusCode::NO_CONTENT))
}
