use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResponse {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
}

pub async fn list() -> Json<Vec<UserResponse>> {
    Json(vec![])
}

pub async fn create(
    Json(request): Json<CreateUserRequest>,
) -> Json<UserResponse> {
    Json(UserResponse {
        id: 1,
        name: request.name,
    })
}
