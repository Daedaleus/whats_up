use std::sync::Arc;
use axum::body::Body;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use crate::repository::AppState;
use crate::service::UserService;

pub fn router() -> Router<Arc<AppState>, Body> {
    Router::new()
        .route("/me", axum::routing::get(get_me))
}

async fn get_me(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user = UserService.get_or_create(state, "asd".into()).await.unwrap();
    Json(user)
}