use axum::{
    Router,
    routing::post,
};

use forgefs_backend::handlers::files::upload_handler;

pub fn file_routes() -> Router {
    Router::new()
        .route("/upload", post(upload_handler))
}
