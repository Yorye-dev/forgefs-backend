//use async_trait::async_trait;
use crate::domain::file::UploadFile;
//use crate::error::AppError;

//#[async_trait]
pub trait FileService: Send + Sync {
    async fn upload_file(&self, file: UploadFile, dest_path :String);
    // async fn move_file(&self, src: String, dst: String) -> Result<(), AppError>;
    // async fn delete_file(&self, path: String) -> Result<(), AppError>;
}
