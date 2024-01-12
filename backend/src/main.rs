use color_eyre::Report;

mod controller;
mod repository;
mod service;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let client = repository::init().await?;
    let state = controller::AppState::new(client.clone(), "whatsup".to_string());

    controller::init([127, 0, 0, 1], 3000, state).await?;
    Ok(())
}
