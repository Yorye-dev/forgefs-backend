mod routes;
mod handlers;
mod services;
//mod infra;

use axum;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    
    dotenv().ok();

    let url = env::var("URL").expect("Error en .env falta URL");
    let project_name = env::var("PROJECT_NAME").expect("Error en .env falta PROJECT_NAMe");

    //handlers::files::pintamos_en_pantalla();

    println!("{} corriendo en: {}", project_name, url);
    // build our application with a single routes

    let app = routes::app_routes();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
