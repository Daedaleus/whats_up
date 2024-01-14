use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use reqwest::header::HeaderMap;

use crate::controller::middleware::keycloak::extract_token;
use crate::controller::users::responses::UserResponse;
use crate::controller::AppState;
use crate::error::*;

/// Get account
#[utoipa::path(get, path = "/api/v1/users/me",
    responses(
        (status = 200, description = "Successfully retrieved account"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    )
)]
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

/// Join group by id
#[utoipa::path(post, path = "/api/v1/users/join",
    responses(
        (status = 202, description = "Successfully joined group"),
        (status = 400, description = "Request body is invalid"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Group not found"),
        (status = 500, description = "Internal server error"),
    ),
    params( ("name" = String, description = "Name of the group to join") )
)]
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
        Err(GroupError::RequestParseError) => StatusCode::BAD_REQUEST,
        Err(GroupError::NotFound) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Toggle ready status
#[utoipa::path(patch, path = "/api/v1/users/ready",
    responses(
        (status = 202, description = "Successfully toggled ready status"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    )
)]
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
