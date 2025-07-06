use axum::{Router, routing::get};

pub fn file_routes() -> Router {
    Router::new()
        .route("/files", get(|| async { "File router" }))
}

