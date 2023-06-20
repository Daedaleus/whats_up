use std::sync::Arc;

mod controller;
mod repository;
mod service;
mod model;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let db = repository::connect().await;

    let state = Arc::new(repository::AppState { db });

    controller::start(state).await?;

    Ok(())
}
