use axum::body::Body;
use axum::extract;
use axum::extract::State;
use axum::handler;
use axum::http;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::Json as JsonResponse;
use axum::response::Response;
use axum::response::{IntoResponse, Json};
use axum::{response::Html, routing::get, routing::post, Router};
use serde_json::json;
use serde::{Deserialize, Serialize};


use axum::extract::Extension;

use crate::db::db_operation;


use crate::web::AppState;
use crate::db;



#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

// 1st handler
async fn h_1() -> Html<&'static str> {
    Html("<h1>API works!</h1>")
}

// 2nd handler
async fn h_2() -> Json<Message> {
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
async fn h_3(Json(request_data): Json<RequestData>) -> JsonResponse<ResponseData> {
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
async fn h_4() -> Result<(Json<serde_json::Value>), StatusCode> {
    // the json! macro is from the serde_json library
    let hello_world = json!({ "hello": "world" });

    // Return a tuple with the JSON response and the desired status code
    Ok(Json(hello_world))
}

// handler 5
// returns a status code 201
async fn h_5() -> Result<impl IntoResponse, StatusCode> {
    println!("Hello!");
    println!("Status Code = {:?}",StatusCode::CREATED);
    Ok(StatusCode::CREATED)
}

//
async fn test_db(state: Extension<AppState>) -> Response<Body> {
    let pool = state.pool.clone();
    let result = db_operation(&pool).await;

    match result {
        Ok(data) => Response::new(Body::from(format!("Test data: {:?}", data))),
        Err(err) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Error: {}", err)))
                .unwrap()
        }
    }
}

//  make sure this is NOT Async !
pub fn routes_comp() -> Router {
    Router::new()
        .route("/hello1", get(h_1))
        .route("/hello2", get(h_2))
        .route("/hello3", post(h_3))
        .route("/hello4", get(h_4))
        .route("/hello5", get(h_5))
        //.route("/test_db", get(test_db))

}
