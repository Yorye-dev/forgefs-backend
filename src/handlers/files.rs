use axum::extract::Multipart;
use axum::response::{Html, IntoResponse};
// Agregar servicio
pub async fn upload_handler (mut multipart: Multipart) -> impl IntoResponse{
    // Implememntar logica ? o llamo al servicio?
    // no me queda miu claro
}

