use crate::services::file_service::FileService;
use crate::domain::file::UploadFile;
use std::error::Error;

pub struct FileServiceImpl;

// [async_trait::async_trait]
impl FileService for FileServiceImpl {
    async fn upload_file(_files: Vec<UploadFile>, _dest_path: String) {
        // llamar a infra
        println!("Subiendo fichero: {}", _dest_path);
        //Ok(())
    }
}

