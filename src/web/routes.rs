use axum::middleware;
use axum::extract::State;
use axum::response::Response;
use axum::{response::Html, routing::get, routing::post, Extension, Router};
use hyper::http::StatusCode;
use axum::Json;
use axum::response::Json as JsonResponse;

use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

// 1st handler
async fn handler_1() -> Html<&'static str> {
    
    Html("<h1>API works!</h1>")
}

// 2nd handler
async fn handler_2() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World JSON!"),
    })
}


// 3rd handler
// Request and response JSON structs.
#[derive(Debug, Serialize, Deserialize)]
struct RequestData {
    // Add fields as needed for your JSON request.
    name: String,
    // Example field, you can customize it according to your needs.
}

#[derive(Debug, Serialize)]
struct ResponseData {
    // Add fields as needed for your JSON response.
    greeting: String,
    // Example field, you can customize it according to your needs.
}

// Handler function for POST requests.
async fn handler_3(Json(request_data): Json<RequestData>) -> JsonResponse<ResponseData> {
    // Process the request data and prepare the response.
    let greeting = format!("Hello, {}!", request_data.name);
    
    // Create and return a JSON response.
    JsonResponse(ResponseData { greeting })
}

//  make sure this is NOT Async !
pub fn routes_comp() -> Router {
    Router::new()
        .route("/hello1", get(handler_1))
        .route("/hello2", get(handler_2))
        .route("/hello3",post(handler_3))
}
