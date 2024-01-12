mod groups;
mod health;
mod middleware;
mod users;

use crate::controller::middleware::keycloak::auth;
use crate::controller::users::logic::users_handler;
use crate::controller::groups::logic::group_handler;
use axum::routing::get;
use axum::Router;
use color_eyre::Report;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub(crate) async fn init(ip: [u8; 4], port: u16, state: AppState) -> Result<(), Report> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let shared_state = Arc::new(state);

    let app = Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .route("/health", get(health::health))
                .nest("/groups", group_handler())
                .nest("/users", users_handler()),
        )
        .with_state(shared_state)
        .layer(axum::middleware::from_fn(auth))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    tracing::info!("Listening on port {}", port);
    axum::Server::bind(&SocketAddr::from((ip, port)))
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) client: mongodb::Client,
    pub(crate) db_name: String,
}

impl AppState {
    pub(crate) fn new(client: mongodb::Client, db_name: String) -> Self {
        AppState { client, db_name }
    }
}
