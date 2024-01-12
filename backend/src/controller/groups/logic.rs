use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::controller::groups::responses::{GroupResponse, InsertOneResultResponse};
use crate::controller::AppState;
use crate::repository;
use crate::repository::group::Group;

pub(crate) fn group_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_groups))
        .route("/", post(create_group))
}

pub(crate) async fn get_groups(State(state): State<Arc<AppState>>) -> Json<Vec<GroupResponse>> {
    let groups = repository::group::get_groups(state.client.clone(), state.db_name.clone())
        .await
        .unwrap_or(Vec::new());
    let groups: Vec<GroupResponse> = groups.iter().map(|group| group.into()).collect();
    Json(groups)
}

pub(crate) async fn create_group(
    State(state): State<Arc<AppState>>,
    Json(group): Json<Group>,
) -> Json<InsertOneResultResponse> {
    let result =
        repository::group::create_group(state.client.clone(), state.db_name.clone(), group)
            .await
            .unwrap();
    Json(result.into())
}
