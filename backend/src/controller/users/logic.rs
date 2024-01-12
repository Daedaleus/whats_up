use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use reqwest::header::HeaderMap;

use crate::controller::middleware::keycloak::extract_token;
use crate::controller::users::responses::UserResponse;
use crate::controller::AppState;
use crate::error::WhatsUpError::{GroupNotFound, GroupRequestParseError};

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

    let user = crate::service::users::get_account(state.clone(), token.clone()).await;
    if user.is_err() {
        return StatusCode::UNAUTHORIZED;
    }

    match crate::service::users::join_group(state.clone(), user.unwrap(), group_req).await {
        Ok(_) => StatusCode::ACCEPTED,
        Err(GroupRequestParseError) => StatusCode::BAD_REQUEST,
        Err(GroupNotFound) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub(crate) async fn toggle_ready(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    let token = match extract_token(headers).await {
        Ok(token) => token,
        Err(_) => return StatusCode::UNAUTHORIZED,
    };

    let user = crate::service::users::get_account(state.clone(), token).await;
    if user.is_err() {
        return StatusCode::UNAUTHORIZED;
    }

    match crate::service::users::toggle_ready(state.clone(), user.unwrap()).await {
        Ok(_) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
