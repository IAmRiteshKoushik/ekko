use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
