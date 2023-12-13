use axum::middleware;
use axum::extract::State;
use axum::response::{Json, Response};
use axum::{response::Html, routing::get, routing::post, Extension, Router};
use hyper::http::StatusCode;
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

async fn handler_1() -> Html<&'static str> {
    
    Html("<h1>API works!</h1>")
}

async fn handler_2() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World JSON!"),
    })
}

//  make sure this is NOT Async !
pub fn routes_comp() -> Router {
    Router::new()
        .route("/hello", get(handler_1))
        .route("/hello2", get(handler_2))
}
