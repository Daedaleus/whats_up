use crate::model::User;
use crate::repository::{AppState, UserRepository};
use std::sync::Arc;

pub struct UserService;

impl UserService {
    pub async fn get_or_create(
        &self,
        state: Arc<AppState>,
        username: String,
    ) -> color_eyre::Result<User> {
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

    pub async fn get_all(&self, state: Arc<AppState>) -> color_eyre::Result<Vec<User>> {
        let users = UserRepository.get_all(state).await?;
        Ok(users)
    }
}
