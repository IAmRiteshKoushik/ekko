use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    pub fn failure(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            data: None,
            error: Some(ApiError {
                code,
                message: message.into(),
            }),
        }
    }

    pub fn with_status(self, status: StatusCode) -> Response
    where
        T: Serialize,
    {
        (status, Json(self)).into_response()
    }
}
