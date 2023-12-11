#![allow(unused)]
use sqlx::PgPool; // this is a Postgres connection pool
use tower_http::services::ServeDir;
use axum::{routing::get, Router};
use tracing::info;

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

    let service = ServeDir::new("assets");

    let app = Router::new()
        .route("/", get(hello_world))
        .nest_service("/assets",service);
    // test assets ~ visit -> http://localhost:3000/assets

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn hello_world() -> &'static str {
    "Hello, world!"
}
