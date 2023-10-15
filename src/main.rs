mod domain;
mod infrastructure;
mod routes;

use std::env;

use dotenvy::dotenv;
use env_logger;
use log::error;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Initialize your repository.
    dotenv().ok();

    // Initialize MongoDB connection
    let uri = match env::var("MONGO.URI") {
        Ok(uri) => uri,
        Err(_) => {
            error!("Error loading env info for MongoDB connection");
            "Error loading env variables to connect to MongoDB".to_owned()
        }
    };

    let repository = infrastructure::mongo::MongoTodoRepository::new(&uri).await;

    println!("Starting server...");
    println!("MongoDB URI: {}", uri);

    // Initialize the logger with a specific log level (e.g., debug).
    // env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    // Define your API routes and handlers, including a health check route.
    let api_routes = routes::routes::todo_routes(repository.clone());

    // Define a health check route.
    let health_check = warp::path("health")
        .map(|| warp::reply::html("Service is healthy"));

    // Combine the API routes and the health check route.
    let routes = api_routes.or(health_check);

    // Start the server.
    let port = 8080;
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;

}
