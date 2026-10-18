use axum::{Json, http::StatusCode, response::Response};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::api::response::ApiResponse;

#[derive(Serialize)]
pub struct UserResponse {
    id: u64,
    name: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let name = self.name.trim();

        if name.is_empty() || !name.chars().all(char::is_alphabetic) {
            return Err(ValidationError::new("invalid_name"));
        }

        Ok(())
    }
}

pub async fn list() -> Response {
    ApiResponse::success(Vec::<UserResponse>::new())
        .into_response(StatusCode::OK)
}

pub async fn create(Json(request): Json<CreateUserRequest>) -> Response {
    if let Err(errors) = request.validate() {
        return ApiResponse::<UserResponse>::failure(
            "VALIDATION_ERROR",
            errors.to_string(),
        )
        .into_response(StatusCode::BAD_REQUEST);
    }

    let user = UserResponse {
        id: 1,
        name: request.name,
    };

    ApiResponse::success(user).into_response(StatusCode::CREATED)
}
