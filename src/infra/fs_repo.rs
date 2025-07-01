pub struct FsFileRepository {
    base_path: PathBuf,
}

#[async_trait]
impl FileRepository for FsFileRepository {
    async fn save_file(&self, path: &str, filename: &str, content: Bytes) -> Result<(), FileError> {
        // lógica para escribir en disco aquí
    }
}
