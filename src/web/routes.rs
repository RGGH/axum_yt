use axum::{response::{Response, IntoResponse}, Json};

#[derive(Serialize)]
struct Message {
    message : String
}

enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>)
}

impl IntoResponse for ApiResponse {
    fn into_response(&self)-> Response {
        match self {
        Response::OK => (StatusCode::OK).into_response(),
        Response::Created => (StatusCode::CREATED).into_response(),
        Response::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
    }
        
}}

async fn handler() -> ApiResponse {
    // Your logic here to determine the appropriate response variant

    // For this example, always return ApiResponse::Ok
    ApiResponse::Ok
}
