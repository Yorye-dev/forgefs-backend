use axum::{Router, routing::get};

// our router
let app = Router::new()
    .route("/", get(root));
// which calls one of these handlers
async fn root() {}
