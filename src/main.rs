mod routes;
mod handlers;
mod services;
//mod infra;
mod domain;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    
    let url = env::var("URL").expect("Error en .env falta URL");
    let project_name = env::var("PROJECT_NAME").expect("Error en .env falta PROJECT_NAMe");
    
    println!("{} corriendo en: {}", project_name, url);
    
    let app = routes::app_routes();
    
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
