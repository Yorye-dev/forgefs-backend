use axum::{Router, routing::get};
use crate::handlers::files::upload_file_handler;

pub fn file_routes() -> Router {
    Router::new()
        .route("/upload", get(upload_file_handler()))

// llamar a la handlers
}
