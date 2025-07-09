pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
}

pub struct UploadFile {
    pub name: String,
    pub content: Vec<u8>,
    pub path: String,
}
