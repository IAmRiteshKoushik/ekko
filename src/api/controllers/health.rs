use axum::{http::StatusCode, response::Response};
use serde::Serialize;

use crate::api::response::ApiResponse;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn check() -> Response {
    ApiResponse::success(HealthResponse { status: "ok" })
        .with_status(StatusCode::OK)
}
