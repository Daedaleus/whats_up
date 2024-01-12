use crate::controller::groups::logic::{create_group, get_groups};
use crate::controller::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub(crate) mod logic;
pub(crate) mod responses;

pub(crate) fn group_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_groups))
        .route("/", post(create_group))
}
