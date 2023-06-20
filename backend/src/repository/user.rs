use std::sync::Arc;

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::model::User;
use crate::repository::AppState;

pub struct UserRepository;

impl UserRepository {
    pub async fn get(&self, state: Arc<AppState>, name: String) -> color_eyre::Result<User> {
        let db: Surreal<Client> = state.db.clone();

        let mut response = db
            .query("SELECT name FROM type::table($table) WHERE name = $name limit 1")
            .bind("users")
            .bind(name)
            .await?;

        let result: Option<User> = response.take(0)?;

        Ok(result.unwrap())
    }

    pub async fn create(&self, state: Arc<AppState>, name: String) -> color_eyre::Result<User> {
        let db: Surreal<Client> = state.db.clone();

        let created: User = db.create("users").content(User { name }).await?;

        Ok(created)
    }

    pub async fn get_all(&self, state: Arc<AppState>) -> color_eyre::Result<Vec<User>> {
        let db: Surreal<Client> = state.db.clone();

        let users: Vec<User> = db.select("users").await?;

        Ok(users)
    }
}
