use std::sync::Arc;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::body::Body;
use axum::extract::State;
use axum::routing::get;
use crate::model::Group;
use crate::repository::AppState;
use crate::service::GroupService;


pub fn router() -> Router<Arc<AppState>, Body> {
    Router::new()
        .route("/", get(get_all).post(create))
}

async fn get_all(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let groups = GroupService.get_all(state).await.unwrap();
    Json(groups)
}

async fn create(State(state): State<Arc<AppState>>, Json(group): Json<Group>) -> impl IntoResponse {
    let group = GroupService.create(state, group).await.unwrap();
    Json(group)
}
