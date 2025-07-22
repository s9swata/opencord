use axum::{
    Router,
    routing::{get, post},
};

use diesel::r2d2::{ConnectionManager, Pool};

use tokio::net::TcpListener;

mod models;
mod routes;
mod schema;
use routes::user::create_user;

#[tokio::main]
async fn main() {
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::new(database_url);
    let db_pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create database connection pool");

    // create TCP listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to bind to address");
    println!("Server running on {}", listener.local_addr().unwrap());

    // compose routes
    let app = Router::new()
        .route("/", get(|| async { "Welcome to the OpenCord Server!" }))
        .route("/user/create", post(create_user))
        .with_state(db_pool);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
