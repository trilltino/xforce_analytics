use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub data: T,
    pub message: String,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            message: message.into(),
        }
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(json!({
            "data": self.data,
            "message": self.message,
        }))
        .into_response()
    }
}

pub fn success<T: Serialize>(data: T, message: impl Into<String>) -> SuccessResponse<T> {
    SuccessResponse::new(data, message)
}
