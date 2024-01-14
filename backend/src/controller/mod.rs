use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use color_eyre::Report;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::controller::groups::group_handler;
use crate::controller::middleware::keycloak::auth;
use crate::controller::users::users_handler;

mod doc;
pub(crate) mod groups;
mod health;
pub(crate) mod middleware;
pub(crate) mod users;

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
                .nest("/groups", group_handler())
                .nest("/users", users_handler()),
        )
        .with_state(shared_state)
        .layer(axum::middleware::from_fn(auth))
        .route("/health", get(health::health))
        .route("/openapi.json", get(doc::openapi))
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
