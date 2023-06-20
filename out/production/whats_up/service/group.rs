use std::sync::Arc;
use crate::model::Group;
use crate::repository::{AppState, GroupRepository};

pub struct GroupService;

impl GroupService {
    pub async fn get_all(&self, state: Arc<AppState>) -> color_eyre::Result<Vec<Group>> {
        let groups = GroupRepository.get_all(state).await?;
        Ok(groups)
    }

    pub async fn create(&self, state: Arc<AppState>, group: Group) -> color_eyre::Result<Group> {
        let group = GroupRepository.create(state, group).await?;
        Ok(group)
    }
}
