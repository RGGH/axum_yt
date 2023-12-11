use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    use tokio::net::TcpListener;

    println!("Server running on http://0.0.0.0:3000");
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
