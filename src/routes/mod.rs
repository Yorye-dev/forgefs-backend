pub mod files;

use axum::Router;

pub fn app_routes() -> Router {
    Router::new()
        .nest("/files", files::file_routes())
}
