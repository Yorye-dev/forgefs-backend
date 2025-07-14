use crate::services::file_service::FileService;
use crate::domain::file::FileUpload;
use crate::error::AppError;

pub struct FileServiceImpl;

#[async_trait::async_trait]
impl FileService for FileServiceImpl {
    async fn upload_file(&self, file: FileUpload) -> Result<(), AppError> {
        // Aquí va la lógica
        println!("Subiendo fichero: {}", file.name);
        Ok(())
    }
}

