use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::controller::groups::responses::{GroupResponse, InsertOneResultResponse};
use crate::controller::AppState;
use crate::repository::group::Group;

pub(crate) fn group_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_groups))
        .route("/", post(create_group))
}

pub(crate) async fn get_groups(State(state): State<Arc<AppState>>) -> Json<Vec<GroupResponse>> {
    let groups = crate::service::groups::get_groups(state.clone()).await;

    Json(groups)
}

pub(crate) async fn create_group(
    State(state): State<Arc<AppState>>,
    Json(group): Json<Group>,
) -> Json<InsertOneResultResponse> {
    let result = crate::service::groups::create_group(state.clone(), group).await;

    Json(result)
}
