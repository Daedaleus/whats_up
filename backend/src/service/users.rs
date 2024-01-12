use std::sync::Arc;

use color_eyre::Report;
use serde::Deserialize;

use crate::controller::AppState;
use crate::repository::user::{create_user, get_user_by_name, User};

#[allow(dead_code)]
#[derive(Deserialize)]
struct UserInfo {
    sub: String,
    email_verified: bool,
    name: String,
    preferred_username: String,
    given_name: String,
    family_name: String,
    email: String,
}

pub(crate) async fn get_account(state: Arc<AppState>, token: String) -> Result<User, Report> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/realms/whats_up/protocol/openid-connect/userinfo")
        .bearer_auth(token)
        .send()
        .await?;
    let userinfo = response.json::<UserInfo>().await?;
    let username = userinfo.preferred_username.clone();

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

pub(crate) async fn toggle_ready(state: Arc<AppState>, user: User) -> Result<bool, Report> {
    let sucess =
        crate::repository::user::toggle_ready(state.client.clone(), state.db_name.clone(), user)
            .await?;
    Ok(sucess)
}
