use axum::{
    Router,
    routing::{get, post},
};

use super::controllers::{health, users};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::check))
        .route("/users", post(users::create))
        .route("/users", get(users::list))
}
