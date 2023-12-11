#![allow(unused)]
use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower_http::services::ServeDir;
use tracing::info;
use crate::web::hello_world;

mod web;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;

    // start logging info
    tracing_subscriber::fmt::init();
    info!("starting server");
    info!("server running on port 3000");

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let service = ServeDir::new("assets");

    let app = Router::new()
        .route("/", get(hello_world))
        .nest_service("/assets", service);
    // to test assets ~ visit -> http://localhost:3000/assets

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
