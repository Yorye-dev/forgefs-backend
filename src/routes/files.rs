use axum::{
    Router, 
   // routing::get,
    routing::{
        post,
        get,
    },
};
use crate::handlers::files::{upload_file_handler,preuba_logs};

pub fn file_routes() -> Router {
    Router::new()
        .route("/upload", post(upload_file_handler))
        .route("/preuba", get(preuba_logs))
}
