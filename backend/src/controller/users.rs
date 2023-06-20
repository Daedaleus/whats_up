use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};

use crate::repository::AppState;
use crate::service::UserService;

pub fn router() -> Router<Arc<AppState>, Body> {
    Router::new()
        .route("/", axum::routing::get(get_all))
        .route("/me", axum::routing::get(get_me))
}

async fn get_me(
    State(state): State<Arc<AppState>>,
    Extension(user_name): Extension<String>,
) -> impl IntoResponse {
    let user = UserService.get_or_create(state, user_name).await.unwrap();
    Json(user)
}

async fn get_all(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = UserService.get_all(state).await.unwrap();
    Json(users)
}
