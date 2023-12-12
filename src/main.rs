/*
docker run -d -p 5432:5432 --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword postgres
my-postgres is the instance name
password = mysecretpassword
user = postgres

to stop the container -> $docker stop my-postgres

*/

#![allow(unused)]
use crate::web::hello_world;
use anyhow::Result; 
use axum::{Router, routing::get, extract::State};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower_http::services::ServeDir;
use tracing::info;

mod web;

#[derive(Clone,Debug)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;

    // start logging info
    tracing_subscriber::fmt::init();
    info!("starting server ✅ ");
    info!("server running on port 3000 ✅");

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:mysecretpassword@localhost:5432".to_string());

    // set up connection pool
    info!("Connect to Postgres");
    let pool_result = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_str)
        .await;

    let pool = match pool_result {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("❌ Error connecting to the database, 
            (if using docker, start the container) : {}", err);
            return;
        }
    };

    // Create an instance of the AppState with the test pool
    let app_state = AppState { pool };
    println!("{:?}", app_state);

    let service = ServeDir::new("assets");

    let app = Router::new()
        .route("/", get(hello_world))
        .nest_service("/assets", service)
        .with_state(app_state);
    // to test assets ~ visit -> http://localhost:3000/assets

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
