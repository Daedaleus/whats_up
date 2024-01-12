use color_eyre::Report;
use mongodb::options::ClientOptions;

pub(crate) mod group;
pub(crate) mod user;

pub(crate) async fn init() -> Result<mongodb::Client, Report> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("backend".to_string());
    client_options.credential = Some(
        mongodb::options::Credential::builder()
            .username(Some("root".to_string()))
            .password(Some("root".to_string()))
            .build(),
    );

    let client = mongodb::Client::with_options(client_options)?;
    Ok(client)
}
