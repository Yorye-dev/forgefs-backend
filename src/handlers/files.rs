use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
};
//use crate::services::file_service::FileService;
use crate::domain::file::UploadFile;

pub fn upload_file_handler (//mut multipart: Multipart
                                 ){

    // 1. Parsear el multipart a un Objeto con nombre y contenido?
    // 2. Obtener las ruta destino.
    // 3. Llamar a la capa de servcio, con un contenido, nombre y ruta destino.
    // 4. Manejar errores y devoer una respeusta.

   println!("Esto esta siendo llamado desde handler");
}

//async fn parse_multipart (multipart :Multipart, )

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
