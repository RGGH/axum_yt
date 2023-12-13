use axum::body::Body;
use axum::extract;
use axum::extract::State;
use axum::http;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::Json as JsonResponse;
use axum::response::Response;
use axum::response::{IntoResponse, Json};
use axum::{response::Html, routing::get, routing::post, Extension, Router};
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::db::db_operation;


use crate::web::AppState;
use crate::db;



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
    bitcoin: i32,
}

// Handler function for POST requests.
// curl -X POST -H "Content-Type: application/json" -d '{"name": "John"}' http://localhost:3000/hello3
async fn handler_3(Json(request_data): Json<RequestData>) -> JsonResponse<ResponseData> {
    // Process the request data and prepare the response.
    let greeting = format!("Hello {}", request_data.name);

    // Create the ResponseData with an additional bitcoin field.
    let response_data = ResponseData {
        greeting,
        bitcoin: 21,
    };

    // Create and return a JSON response.
    JsonResponse(response_data)
}

// handler 4 
// curl -X GET http://localhost/hello4
async fn handler_4() -> Result<(Json<serde_json::Value>), StatusCode> {
    // the json! macro is from the serde_json library
    let hello_world = json!({ "hello": "world" });

    // Return a tuple with the JSON response and the desired status code
    Ok(Json(hello_world))
}

// handler 5
// returns a status code 201
async fn handler_5() -> Result<impl IntoResponse, StatusCode> {
    println!("Hello!");
    println!("Status Code = {:?}",StatusCode::CREATED);
    Ok(StatusCode::CREATED)
}

//
async fn test_db(state: extract::Extension<AppState>) -> http::Response<Body> {
    // Access the database pool from the state
    let pool = state.pool.clone();

    // Use the pool in your database operations
    let result = db_operation(&pool).await;

    // Handle the result and build the response accordingly
    match result {
        Ok(data) => {
            // Do something with the data
            http::Response::new(Body::from(format!("Test data: {:?}", data)))
        }
        Err(err) => {
            // Handle the error
            http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Error: {}", err)))
                .unwrap()
        }
    }
}

//  make sure this is NOT Async !
pub fn routes_comp() -> Router {
    Router::new()
        .route("/hello1", get(handler_1))
        .route("/hello2", get(handler_2))
        .route("/hello3", post(handler_3))
        .route("/hello4", get(handler_4))
        .route("/hello5", get(handler_5))
}
