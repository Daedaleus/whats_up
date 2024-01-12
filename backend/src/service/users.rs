use std::sync::Arc;

use crate::controller::middleware::keycloak::get_user_name_from_info;
use crate::controller::users::responses::JoinGroupRequest;
use crate::controller::AppState;
use crate::error::*;
use crate::repository::group::get_group_by_name;
use crate::repository::user::{add_group_to_user, create_user, get_user_by_name, User};
use color_eyre::Report;

pub(crate) async fn get_account(state: Arc<AppState>, token: String) -> Result<User, Report> {
    let username = get_user_name_from_info(token).await?;

    let user = get_user_by_name(
        state.client.clone(),
        state.db_name.clone(),
        username.clone(),
    )
    .await;
    match user {
        Ok(user) => Ok(user),
        Err(_) => {
            let user = User::new(username);
            let created_user =
                create_user(state.client.clone(), state.db_name.clone(), user).await?;
            Ok(created_user)
        }
    }
}

pub(crate) async fn join_group(
    state: Arc<AppState>,
    user: User,
    group_req: String,
) -> Result<(), GroupError> {
    let group_req: JoinGroupRequest = match serde_json::from_str(&group_req) {
        Ok(group_req) => group_req,
        Err(_) => return Err(GroupError::RequestParseError),
    };

    let group = get_group_by_name(
        state.client.clone(),
        state.db_name.clone(),
        group_req.name.clone(),
    )
    .await;
    if group.is_err() {
        return Err(GroupError::NotFound);
    }
    let result = add_group_to_user(
        state.client.clone(),
        state.db_name.clone(),
        user,
        group.unwrap(),
    )
    .await;

    if result.is_err() {
        return Err(GroupError::AddError);
    }

    Ok(())
}

pub(crate) async fn toggle_ready(state: Arc<AppState>, user: User) -> Result<(), Report> {
    crate::repository::user::toggle_ready(state.client.clone(), state.db_name.clone(), user)
        .await?;
    Ok(())
}
