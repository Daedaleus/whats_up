use std::sync::Arc;

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::model::Group;
use crate::repository::AppState;

pub struct GroupRepository;

impl GroupRepository {
    pub async fn get_all(&self, state: Arc<AppState>) -> color_eyre::Result<Vec<Group>> {
        let db: Surreal<Client> = state.db.clone();

        let groups: Vec<Group> = db.select("groups").await?;
        Ok(groups)
    }

    pub async fn create(&self, state: Arc<AppState>, group: Group) -> color_eyre::Result<Group> {
        let db: Surreal<Client> = state.db.clone();

        let created: Group = db.create("groups").content(Group {
            name: group.name,
            description: group.description,
        }).await?;

        Ok(created)
    }
}