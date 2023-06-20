use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub use group::*;
pub use user::*;

mod group;
mod user;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Client>,
}

pub async fn connect() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "password1",
    })
        .await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    db
}