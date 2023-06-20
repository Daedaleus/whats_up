use std::sync::Arc;
use crate::model::User;
use crate::repository::{AppState, UserRepository};

pub struct UserService;

impl UserService {

    pub async fn get_or_create(&self, state: Arc<AppState>, username: String) -> color_eyre::Result<User> {
        let found_user = UserService.get(state.clone(), username.clone()).await;
        match found_user {
            Ok(user) => Ok(user),
            Err(_) => {
                let new_user = UserService.create(state, username).await?;
                Ok(new_user)
            }
        }

    }

    async fn get(&self, state: Arc<AppState>, username: String) -> color_eyre::Result<User> {
        let user = UserRepository.get(state, username).await?;
        Ok(user)
    }

    async fn create(&self, state: Arc<AppState>, username: String) -> color_eyre::Result<User> {
        let user = UserRepository.create(state, username).await?;
        Ok(user)
    }
}