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

pub async fn health() -> Json<Health> {
    let status = Health::default();
    Json(status)
}
