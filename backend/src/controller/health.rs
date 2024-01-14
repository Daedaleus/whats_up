use axum::Json;

#[derive(serde::Serialize)]
pub struct Health {
    healthy: bool,
}

impl Default for Health {
    fn default() -> Self {
        Health { healthy: true }
    }
}

#[utoipa::path(get, path = "/health",
    responses(
        (status = 200, description = "Successfully retrieved health status"),
        (status = 500, description = "Internal server error"),
    )
)]
pub async fn health() -> Json<Health> {
    let status = Health::default();
    Json(status)
}
