use crate::repository::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub(crate) struct JoinGroupRequest {
    pub(crate) name: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    id: Option<String>,
    name: String,
    groups: Vec<String>,
    ready: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.map(|id| id.to_string()),
            name: user.name,
            groups: user
                .groups
                .iter()
                .map(|group| group.to_string())
                .collect::<Vec<String>>(),
            ready: user.ready,
        }
    }
}
