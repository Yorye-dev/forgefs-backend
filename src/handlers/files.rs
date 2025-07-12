use axum::{
    extract::{Multipart, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use base64::engine::general_purpose;
use base64::Engine;
use log::{info, warn};

//use crate::services::file_service::FileService;
use crate::domain::file::{UploadFile, UploadQuery};

pub async fn upload_file_handler (query: Query<UploadQuery>, multipart: Multipart) -> impl  IntoResponse{

    let _files :Vec<UploadFile> = parse_multipart(multipart).await;
    let _dest_path :String = query.dest_path.clone();
    // 1. Parsear el multipart a un Objeto con nombre y contenido? - esto se teine que delar a una
    //    funcion que retorne ya la struc completa:
    // 
    // 2. Obtener las ruta destino.
    // 3. Llamar a la capa de servcio, con un contenido, nombre y ruta destino.
    // 4. Manejar errores y devoer una respeusta.
    //
    println!("Esto es el path {} y esto el nombre del fichero ...", _dest_path,  );

    Json("ok")
}

pub async fn preuba_logs() -> impl IntoResponse {
    println!("📢 Se llamó al handler");
    warn!("Pete");
    info!("Info");
    Json("ok")
}

async fn parse_multipart(mut multipart: Multipart) -> Vec<UploadFile> {

    let mut files_info = Vec::new();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unnamed").to_string();
        let data = field.bytes().await.unwrap();

        files_info.push(UploadFile {
            name,
            size: data.len(),
            data: general_purpose::STANDARD.encode(&data),
        });
    }

    files_info
}

//async fn parse_multipart_to_ (multipart :Multipart, )

//pub async fn upload_handler (mut multipart: Multipart) {//-> impl IntoResponse{
    // 1. Parsear el multipart  (nomber, contenido)

   // while let Some(field) = multipart.next_field().await.unwrap() {
      //  let name = field.name().unwrap().to_string();
     //   let data = field.bytes().await.unwrap();

    //    println!("Length of `{}` is {} bytes", name, data.len());
    //}
    // 2. Obtener ruta destino (desde un query param)
    // 3. Llamar al servcio, file_service(path, file_bytes)
    // 4. Manejar errores, y devolver una respusta
//}


//pub async fn upload_file_handler(
  //  mut multipart: Multipart,
  //
//) -> impl IntoResponse {
  //  let file_service = FileService::new(); // O injectado si usas Arc

    //match file_service.upload_file(multipart).await {
      //  Ok(_) => StatusCode::OK,
       // Err(err) => {
       //     eprintln!("Error al subir archivo: {:?}", err);
       //     StatusCode::INTERNAL_SERVER_ERROR
       // }
   // }
//}
