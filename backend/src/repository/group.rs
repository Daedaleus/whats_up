use color_eyre::Report;
use futures::stream::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) name: String,
}

pub(crate) async fn get_groups(client: Client, db_name: String) -> Result<Vec<Group>, Report> {
    let database = client.database(&db_name);
    let groups = database.collection::<Group>("groups");

    let mut cursor = groups.find(None, None).await?;
    let mut result = Vec::new();
    while let Some(group) = cursor.next().await {
        result.push(group?);
    }
    Ok(result)
}

pub(crate) async fn get_group_by_name(
    client: Client,
    db_name: String,
    name: String,
) -> Result<Group, Report> {
    let database = client.database(&db_name);
    let groups = database.collection::<Group>("groups");

    let filter = mongodb::bson::doc! { "name": name };
    let result = groups.find_one(filter, None).await?;
    match result.is_some() {
        true => Ok(result.unwrap()),
        false => Err(Report::msg("Group not found")),
    }
}

pub(crate) async fn create_group(
    client: Client,
    db_name: String,
    group: Group,
) -> Result<InsertOneResult, Report> {
    let database = client.database(&db_name);
    let groups = database.collection::<Group>("groups");

    let result = groups.insert_one(group, None).await?;
    Ok(result)
}
