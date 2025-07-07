use axum::{Router, routing::get};
use crate::handlers::files::pintamos_en_pantalla;

pub fn file_routes() -> Router {
    Router::new()
        .route("/upload", get(pintamos_en_pantalla()))

// llamar a la handlers
}
