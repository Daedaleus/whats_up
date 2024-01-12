use crate::controller::groups::responses::{GroupResponse, InsertOneResultResponse};
use crate::controller::AppState;
use crate::repository;
use crate::repository::group::Group;
use std::sync::Arc;

pub(crate) async fn get_groups(state: Arc<AppState>) -> Vec<GroupResponse> {
    let groups = repository::group::get_groups(state.client.clone(), state.db_name.clone())
        .await
        .unwrap_or(Vec::new());

    groups.iter().map(|group| group.into()).collect()
}

pub(crate) async fn create_group(state: Arc<AppState>, group: Group) -> InsertOneResultResponse {
    let result =
        repository::group::create_group(state.client.clone(), state.db_name.clone(), group)
            .await
            .unwrap();
    result.into()
}
