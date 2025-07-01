#[async_trait]
pub trait FileRepository {
    async fn save_file(&self, path: &str, filename: &str, content: Bytes) -> Result<(), FileError>;
}

