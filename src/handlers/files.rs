//use axum::extract::Multipart;

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

pub fn pintamosEnPantalla(){
    println!("Esto esta siendo llamado desde handler");
}

