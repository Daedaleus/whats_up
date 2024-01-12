use mongodb::results::InsertOneResult;
use serde::Serialize;

use crate::repository::group::Group;

#[derive(Serialize)]
pub struct GroupResponse {
    id: Option<String>,
    name: String,
}

impl From<Group> for GroupResponse {
    fn from(group: Group) -> Self {
        Self {
            id: group.id.map(|id| id.to_string()),
            name: group.name,
        }
    }
}

impl From<&Group> for GroupResponse {
    fn from(group: &Group) -> Self {
        let id = group.id.clone().map(|id| id.to_string());
        let name = group.name.clone();
        Self { id, name }
    }
}

#[derive(Serialize)]
pub struct InsertOneResultResponse {
    inserted_id: String,
}

impl From<InsertOneResult> for InsertOneResultResponse {
    fn from(result: InsertOneResult) -> Self {
        Self {
            inserted_id: result.inserted_id.as_object_id().unwrap().to_string(),
        }
    }
}
