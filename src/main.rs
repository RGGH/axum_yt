/*
docker run -d -p 5432:5432 --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword postgres
my-postgres is the instance name
password = mysecretpassword
user = postgres

to stop the container -> $docker stop my-postgres

*/

#![allow(unused)]
use crate::web::routes_comp;
use crate::db::init_db_pool;
use anyhow::Result;
use axum::response::Html;
use axum::{extract::State, routing::get, Router};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tower_http::services::ServeDir;
use tracing::info;

mod web;
mod db;

#[derive(Clone, Debug)]
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
    
    // Create an instance of the AppState with the test pool
    // let app_state = web::AppState{ pool };
  
    // set directory for static
    let service = ServeDir::new("assets");

    let app = Router::new()
        .merge(routes_comp())
        .nest_service("/assets", service);
    // to test assets ~ visit -> http://localhost:3000/assets

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
