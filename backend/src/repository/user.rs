use color_eyre::Report;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};

use crate::repository::group::Group;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) name: String,
    pub(crate) groups: Vec<ObjectId>,
    pub(crate) ready: bool,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            name: "".to_string(),
            groups: vec![],
            ready: false,
        }
    }
}

impl User {
    pub(crate) fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub(crate) fn new(name: String) -> Self {
        User::default().with_name(name)
    }
}

pub(crate) async fn toggle_ready(
    client: Client,
    db_name: String,
    user: User,
) -> Result<bool, Report> {
    let database = client.database(&db_name);
    let users = database.collection::<User>("users");
    let filter = doc! { "_id": user.id };
    let update = doc! { "$set": { "ready": !user.ready } };
    let result = users.update_one(filter, update, None).await?;
    match result.modified_count {
        1 => Ok(true),
        _ => Err(Report::msg("User not found")),
    }
}

pub(crate) async fn get_user_by_name(
    client: Client,
    db_name: String,
    name: String,
) -> Result<User, Report> {
    let database = client.database(&db_name);
    let users = database.collection::<User>("users");
    let filter = doc! { "name": name };
    let result = users.find_one(filter, None).await?;
    match result.is_some() {
        true => Ok(result.unwrap()),
        false => Err(Report::msg("User not found")),
    }
}

pub(crate) async fn add_group_to_user(
    client: Client,
    db_name: String,
    user: User,
    group: Group,
) -> Result<User, Report> {
    let database = client.database(&db_name);
    let users = database.collection::<User>("users");
    let filter = doc! { "_id": user.id };
    let update = doc! { "$push": { "groups": group.id } };
    let result = users.update_one(filter, update, None).await?;
    match result.modified_count {
        1 => Ok(user),
        _ => Err(Report::msg("User not found")),
    }
}

pub(crate) async fn create_user(
    client: Client,
    db_name: String,
    user: User,
) -> Result<User, Report> {
    tracing::info!("create_user");
    let database = client.database(&db_name);
    let groups = database.collection::<User>("users");
    groups.insert_one(user.clone(), None).await?;
    get_user_by_name(client, db_name, user.name).await
}
