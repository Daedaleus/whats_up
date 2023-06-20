use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::repository::AppState;

mod groups;
mod middleware;
mod users;

pub async fn start(state: Arc<AppState>) -> color_eyre::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let api_v1 = Router::new()
        .nest("/groups", groups::router())
        .nest("/users", users::router());

    let app = Router::new()
        .nest("/api/v1", api_v1)
        .with_state(state)
        .layer(cors)
        .layer(axum::middleware::from_fn(middleware::jwt::auth));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
