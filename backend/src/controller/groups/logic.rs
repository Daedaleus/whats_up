use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::controller::groups::responses::{GroupResponse, InsertOneResultResponse};
use crate::controller::AppState;
use crate::repository::group::Group;

#[utoipa::path(get, path = "/api/v1/groups",
    responses(
        (status = 200, description = "Successfully retrieved groups"),
        (status = 500, description = "Internal server error"),
    )
)]
pub(crate) async fn get_groups(State(state): State<Arc<AppState>>) -> Json<Vec<GroupResponse>> {
    let groups = crate::service::groups::get_groups(state.clone()).await;

    Json(groups)
}

#[utoipa::path(post, path = "/api/v1/groups",
    responses(
        (status = 201, description = "Successfully created group"),
        (status = 400, description = "Request body is invalid"),
        (status = 500, description = "Internal server error"),
    ),
    params( ("name" = String, description = "Name of the group to create") )
)]
pub(crate) async fn create_group(
    State(state): State<Arc<AppState>>,
    Json(group): Json<Group>,
) -> Json<InsertOneResultResponse> {
    let result = crate::service::groups::create_group(state.clone(), group).await;

    Json(result)
}
