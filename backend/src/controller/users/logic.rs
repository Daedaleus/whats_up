use crate::controller::middleware::keycloak::extract_token;
use crate::controller::users::responses::{JoinGroupRequest, UserResponse};
use crate::controller::AppState;
use crate::repository::group::get_group_by_name;
use crate::repository::user::add_group_to_user;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use reqwest::header::HeaderMap;
use std::sync::Arc;

pub(crate) fn users_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(get_account))
        .route("/join", post(join_group))
        .route("/ready", patch(toggle_ready))
}

pub(crate) async fn get_account(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserResponse>, StatusCode> {
    let token = match extract_token(headers).await {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    match crate::service::users::get_account(state, token).await {
        Ok(user) => Ok(Json(user.into())),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub(crate) async fn join_group(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    group_req: String,
) -> StatusCode {
    let token = match extract_token(headers).await {
        Ok(token) => token,
        Err(_) => return StatusCode::UNAUTHORIZED,
    };

    let user = crate::service::users::get_account(state.clone(), token).await;
    if user.is_err() {
        return StatusCode::UNAUTHORIZED;
    }
    let user = user.unwrap();
    let group_req: JoinGroupRequest = match serde_json::from_str(&group_req) {
        Ok(group_req) => group_req,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let group = get_group_by_name(
        state.client.clone(),
        state.db_name.clone(),
        group_req.name.clone(),
    )
    .await;
    if group.is_err() {
        return StatusCode::NOT_FOUND;
    }
    let result = add_group_to_user(
        state.client.clone(),
        state.db_name.clone(),
        user.clone(),
        group.unwrap(),
    )
    .await;

    if result.is_err() {
        return StatusCode::NOT_FOUND;
    }

    StatusCode::ACCEPTED
}

pub(crate) async fn toggle_ready(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    let auth_header = headers.get("Authorization");
    if auth_header.is_none() {
        return StatusCode::UNAUTHORIZED;
    }
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let token = token.replace("Bearer ", "");
    let user = crate::service::users::get_account(state.clone(), token).await;
    if user.is_err() {
        return StatusCode::UNAUTHORIZED;
    }
    let user = user.unwrap();
    let result = crate::service::users::toggle_ready(state.clone(), user).await;
    if result.is_err() {
        return StatusCode::UNAUTHORIZED;
    }
    StatusCode::ACCEPTED
}
