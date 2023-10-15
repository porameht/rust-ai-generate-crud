use warp::Filter;
use crate::infrastructure::mongo::MongoTodoRepository;

use super::handlers::{create_todo_handler, get_todos_handler, get_todo_handler, update_todo_handler, delete_todo_handler};

pub fn with_repository(repository: MongoTodoRepository) -> impl Filter<Extract = (MongoTodoRepository,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || repository.clone())
}

pub fn todo_routes(repository: MongoTodoRepository) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let create_todo = warp::post()
        .and(warp::path("todos"))
        .and(warp::body::json())
        .and(with_repository(repository.clone()))
        .and_then(create_todo_handler);

    let get_todos = warp::get()
        .and(warp::path("todos"))
        .and(with_repository(repository.clone()))
        .and_then(get_todos_handler);

    let get_todo = warp::get()
        .and(warp::path("todo").and(warp::path::param()))
        .and(with_repository(repository.clone()))
        .and_then(get_todo_handler);

    let update_todo = warp::put()
        .and(warp::path("todo").and(warp::path::param()))
        .and(warp::body::json())
        .and(with_repository(repository.clone()))
        .and_then(update_todo_handler);

    let delete_todo = warp::delete()
        .and(warp::path("todo").and(warp::path::param()))
        .and(with_repository(repository))
        .and_then(delete_todo_handler);

    create_todo.or(get_todos).or(get_todo).or(update_todo).or(delete_todo)
}
