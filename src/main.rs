mod routes;

use axum::{
    routing::get,
    Router
};
use dotenv::dotenv;
use std::env;

use routes::file_routes;

#[tokio::main]
async fn main() {
    
    dotenv().ok();

    let url = env::var("URL").expect("Error en .env falta URL");
    let project_name = env::var("PROJECT_NAME").expect("Error en .env falta PROJECT_NAMe");

    println!("{} corriendo en: {}", project_name, url);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/files", file_routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
