use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait FileRepository {

    async fn save_file(&self, path: &str, filename: &str, content: Vec<u8>) -> Result<(), FileError>;
}

