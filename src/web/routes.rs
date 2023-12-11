use axum::{response::{Response, IntoResponse}, Json};
use serde::Serialize;
use hyper::http::StatusCode;

#[derive(Serialize)]
struct Message {
    message : String
}

enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>)
}

// // Implement IntoResponse for ApiResponse
// impl IntoResponse for ApiResponse {
//     fn into_response(self) -> Response {
//         match self {
//             ApiResponse::OK => Response::new(StatusCode::OK),
//             ApiResponse::Created => Response::new(StatusCode::CREATED),
//             ApiResponse::JsonData(data) => Response::new(StatusCode::OK)
//                 .set_header("Content-Type", "application/json")
//                 .set_body(data),
//         }
//     }
// }

// async fn handler() -> ApiResponse {
//     // Your logic here to determine the appropriate response variant

//     // For this example, always return ApiResponse::Ok
//     ApiResponse::Ok
// }



// Basic handler that responds with a static string
pub async fn hello_world() -> &'static str {


    "Hello, world!"
}
