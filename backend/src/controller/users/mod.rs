use crate::controller::users::logic::{get_account, join_group, toggle_ready};
use crate::controller::AppState;
use axum::routing::{get, patch, post};
use axum::Router;
use std::sync::Arc;

pub(crate) mod logic;
pub(crate) mod responses;

pub(crate) fn users_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(get_account))
        .route("/join", post(join_group))
        .route("/ready", patch(toggle_ready))
}
