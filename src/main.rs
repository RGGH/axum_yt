use axum::{routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;

    tracing_subscriber::fmt::init();
    info!("starting server");

    let app = Router::new().route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}


